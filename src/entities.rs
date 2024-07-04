use std::f32::consts::{PI, TAU};

use wasm_bindgen::prelude::*;

use crate::{
    recycled_list::{RecycledList, RecycledListItem, RecycledListRef},
    utils::{distance, to_creep_position, FloatPosition, GridPosition},
    State,
};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct WalkingProgress {
    pub current_goal: u32,
    pub steps_taken: u32,
}

#[derive(Copy, Clone)]
#[wasm_bindgen]
pub enum CreepKind {
    Normal,
    Grouped,
    Speed,
    Big,
}

#[derive(Clone, Copy)]
pub struct Creep {
    pub pos: FloatPosition,
    pub health: f32,
    pub max_health: f32,
    pub walking: WalkingProgress,
    pub speed: u32, // no. of ticks to walk one grid cell, lower is faster
    pub gold: u32,
    pub kind: CreepKind,
}

#[derive(Copy, Clone)]
pub struct StaticBasicData {
    pub range: f32, // tiles
    pub damage: f32,
    pub attack_speed: f32,      // attacks/s
    pub rotation_speed: f32,    // deg/s
    pub projectile_speed: f32,  // tiles/s
    pub damage_multiplier: f32, // 100% = normal damage
    pub cost: u32,
}

pub const BASIC: [StaticBasicData; 11] = [
    // 0
    StaticBasicData {
        range: 2.0,
        damage: 3.0,
        attack_speed: 1.25,
        rotation_speed: 90.0,
        projectile_speed: 2.8,
        damage_multiplier: 100.0,
        cost: 20,
    },
    // 1
    StaticBasicData {
        range: 2.2,
        damage: 9.4,
        attack_speed: 1.4,
        rotation_speed: 90.0,
        projectile_speed: 3.0,
        damage_multiplier: 100.0,
        cost: 50,
    },
    // 2
    StaticBasicData {
        range: 2.3,
        damage: 11.8,
        attack_speed: 1.4,
        rotation_speed: 110.0,
        projectile_speed: 3.2,
        damage_multiplier: 100.0,
        cost: 26,
    },
    // 3
    StaticBasicData {
        range: 2.3,
        damage: 14.5,
        attack_speed: 1.55,
        rotation_speed: 120.0,
        projectile_speed: 3.3,
        damage_multiplier: 100.0,
        cost: 42,
    },
    // 4
    StaticBasicData {
        range: 2.5,
        damage: 18.0,
        attack_speed: 1.55,
        rotation_speed: 120.0,
        projectile_speed: 3.5,
        damage_multiplier: 100.0,
        cost: 61,
    },
    // 5
    StaticBasicData {
        range: 2.2,
        damage: 23.0,
        attack_speed: 1.7,
        rotation_speed: 135.0,
        projectile_speed: 3.5,
        damage_multiplier: 100.0,
        cost: 90,
    },
    // 6
    StaticBasicData {
        range: 2.6,
        damage: 27.0,
        attack_speed: 1.7,
        rotation_speed: 145.0,
        projectile_speed: 3.7,
        damage_multiplier: 100.0,
        cost: 150,
    },
    // 7
    StaticBasicData {
        range: 2.7,
        damage: 33.5,
        attack_speed: 1.95,
        rotation_speed: 145.0,
        projectile_speed: 3.8,
        damage_multiplier: 100.0,
        cost: 250,
    },
    // 8
    StaticBasicData {
        range: 2.7,
        damage: 41.0,
        attack_speed: 2.05,
        rotation_speed: 170.0,
        projectile_speed: 3.9,
        damage_multiplier: 100.0,
        cost: 420,
    },
    // 9
    StaticBasicData {
        range: 2.9,
        damage: 49.0,
        attack_speed: 2.05,
        rotation_speed: 180.0,
        projectile_speed: 4.0,
        damage_multiplier: 100.0,
        cost: 690,
    },
    // 10
    StaticBasicData {
        range: 3.0,
        damage: 57.0,
        attack_speed: 2.3,
        rotation_speed: 180.0,
        projectile_speed: 4.2,
        damage_multiplier: 100.0,
        cost: 1100,
    },
];

#[derive(Copy, Clone)]
pub struct DynamicBasicData {
    pub rotation: f32, // orientation/angle in RAD
    pub target: RecycledListRef,
}

impl FollowsTarget for DynamicBasicData {
    fn get_target(&self) -> RecycledListRef {
        self.target
    }

    fn set_target(&mut self, target: RecycledListRef) {
        self.target = target;
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn set_rotation(&mut self, new_rotation: f32) {
        self.rotation = new_rotation;
    }

    fn get_rotation_speed(&self, level: u32) -> f32 {
        BASIC[level as usize].rotation_speed
    }

    fn get_range(&self, level: u32) -> f32 {
        BASIC[level as usize].range
    }

    fn blast(&mut self, general_data: &mut GeneralData, state: &mut State, is_in_aim: bool) {
        let turret_data = &BASIC[general_data.level as usize];
        if is_in_aim
            && state.tick > general_data.last_shot + (60.0 / turret_data.attack_speed) as u32
        {
            // the turret position is the start of the barrel, where particles are emitted
            let x = (general_data.pos.x as f32 + 0.5) * state.cell_length
                + state.cell_length / 2.0 * self.rotation.cos();
            let y = (general_data.pos.y as f32 + 0.5) * state.cell_length
                + state.cell_length / 2.0 * self.rotation.sin();
            let turret_pos = FloatPosition { x, y };

            general_data.last_shot = state.tick;
            state.particles.add(Particle {
                pos: turret_pos,
                target: self.target,
                damage: turret_data.damage * turret_data.damage_multiplier / 100.0,
                speed: turret_data.projectile_speed * state.cell_length / 60.0,
                explosion_radius: 0.0,
            });
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct StaticSniperData {
    pub range: f32, // tiles
    pub damage: f32,
    pub attack_speed: f32,   // attacks/s
    pub rotation_speed: f32, // deg/s
    pub aiming_speed: f32,   // shooting: 100 / (Aiming Speed) seconds
    pub crit_chance: f32,
    pub crit_multiplier: f32,
    pub cost: u32,
}

pub const SNIPER: [StaticSniperData; 11] = [
    // 0
    StaticSniperData {
        range: 4.5,
        damage: 46.0,
        attack_speed: 0.28,
        rotation_speed: 50.0,
        aiming_speed: 90.0,
        crit_chance: 0.15,
        crit_multiplier: 1.5,
        cost: 80,
    },
    // 1
    StaticSniperData {
        range: 4.9,
        damage: 64.0,
        attack_speed: 0.32,
        rotation_speed: 55.0,
        aiming_speed: 100.0,
        crit_chance: 0.15,
        crit_multiplier: 1.5,
        cost: 42,
    },
    // 2
    StaticSniperData {
        range: 5.3,
        damage: 84.0,
        attack_speed: 0.38,
        rotation_speed: 60.0,
        aiming_speed: 105.0,
        crit_chance: 0.15,
        crit_multiplier: 1.5,
        cost: 72,
    },
    // 3
    StaticSniperData {
        range: 5.3,
        damage: 128.0,
        attack_speed: 0.38,
        rotation_speed: 64.0,
        aiming_speed: 110.0,
        crit_chance: 0.15,
        crit_multiplier: 1.5,
        cost: 110,
    },
    // 4
    StaticSniperData {
        range: 5.7,
        damage: 180.0,
        attack_speed: 0.44,
        rotation_speed: 73.0,
        aiming_speed: 120.0,
        crit_chance: 0.15,
        crit_multiplier: 1.5,
        cost: 176,
    },
    // 5
    StaticSniperData {
        range: 6.1,
        damage: 250.0,
        attack_speed: 0.5,
        rotation_speed: 73.0,
        aiming_speed: 135.0,
        crit_chance: 0.15,
        crit_multiplier: 1.5,
        cost: 380,
    },
    // 6
    StaticSniperData {
        range: 6.1,
        damage: 310.0,
        attack_speed: 0.53,
        rotation_speed: 81.0,
        aiming_speed: 135.0,
        crit_chance: 0.15,
        crit_multiplier: 1.5,
        cost: 540,
    },
    // 7
    StaticSniperData {
        range: 6.5,
        damage: 430.0,
        attack_speed: 0.53,
        rotation_speed: 86.0,
        aiming_speed: 150.0,
        crit_chance: 0.15,
        crit_multiplier: 1.5,
        cost: 760,
    },
    // 8
    StaticSniperData {
        range: 6.9,
        damage: 560.0,
        attack_speed: 0.58,
        rotation_speed: 86.0,
        aiming_speed: 170.0,
        crit_chance: 0.15,
        crit_multiplier: 1.5,
        cost: 1140,
    },
    // 9
    StaticSniperData {
        range: 7.3,
        damage: 700.0,
        attack_speed: 0.62,
        rotation_speed: 86.0,
        aiming_speed: 180.0,
        crit_chance: 0.15,
        crit_multiplier: 1.5,
        cost: 1800,
    },
    // 10
    StaticSniperData {
        range: 7.8,
        damage: 920.0,
        attack_speed: 0.65,
        rotation_speed: 90.0,
        aiming_speed: 200.0,
        crit_chance: 0.15,
        crit_multiplier: 1.5,
        cost: 3000,
    },
];

#[derive(Copy, Clone)]
pub struct DynamicSniperData {
    pub rotation: f32, // orientation/angle in RAD
    pub target: RecycledListRef,
    pub aiming_ticks: u32, // see StaticSniperData::aiming_speed
}

impl FollowsTarget for DynamicSniperData {
    fn get_target(&self) -> RecycledListRef {
        self.target
    }

    fn set_target(&mut self, target: RecycledListRef) {
        self.aiming_ticks = 0;
        self.target = target;
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn set_rotation(&mut self, new_rotation: f32) {
        self.rotation = new_rotation;
    }

    fn get_rotation_speed(&self, level: u32) -> f32 {
        SNIPER[level as usize].rotation_speed
    }

    fn get_range(&self, level: u32) -> f32 {
        SNIPER[level as usize].range
    }

    fn blast(&mut self, general_data: &mut GeneralData, state: &mut State, is_in_aim: bool) {
        if is_in_aim {
            self.aiming_ticks += 1;
        } else {
            self.aiming_ticks = 0;
            return;
        }

        let turret_data = &SNIPER[general_data.level as usize];
        if is_in_aim
            && state.tick > general_data.last_shot + (60.0 / turret_data.attack_speed) as u32
            && self.aiming_ticks > ((100.0 / turret_data.aiming_speed) * 60.0) as u32
        {
            general_data.last_shot = state.tick;
            self.aiming_ticks = 0;

            let mut_target_creep = state.creeps.get_mut(self.target).unwrap();

            state.sniper_particles.add(SniperParticle {
                start_pos: to_creep_position(general_data.pos, state.cell_length),
                target_pos: mut_target_creep.pos,
                lifetime_in_ticks: 5,
            });

            mut_target_creep.health -= turret_data.damage;
            if mut_target_creep.health <= 0.0 {
                state.gold += mut_target_creep.gold;
                state.creeps.remove(self.target);
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct StaticCannonData {
    pub range: f32, // tiles
    pub damage: f32,
    pub explosion_radius: f32,
    pub attack_speed: f32,     // attacks/s
    pub rotation_speed: f32,   // deg/s
    pub projectile_speed: f32, // tiles/s
    pub cost: u32,
}

pub const CANNON: [StaticCannonData; 11] = [
    // 0
    StaticCannonData {
        range: 1.7,
        damage: 14.0,
        explosion_radius: 0.6,
        attack_speed: 0.6,
        rotation_speed: 40.0,
        projectile_speed: 2.2,
        cost: 60,
    },
    // 1
    StaticCannonData {
        range: 1.9,
        damage: 22.7,
        explosion_radius: 0.75,
        attack_speed: 0.7,
        rotation_speed: 50.0,
        projectile_speed: 2.4,
        cost: 42,
    },
    // 2
    StaticCannonData {
        range: 2.0,
        damage: 30.2,
        explosion_radius: 0.9,
        attack_speed: 0.7,
        rotation_speed: 60.0,
        projectile_speed: 2.6,
        cost: 63,
    },
    // 3
    StaticCannonData {
        range: 2.15,
        damage: 39.8,
        explosion_radius: 1.05,
        attack_speed: 0.85,
        rotation_speed: 70.0,
        projectile_speed: 2.6,
        cost: 115,
    },
    // 4
    StaticCannonData {
        range: 2.15,
        damage: 52.9,
        explosion_radius: 1.2,
        attack_speed: 1.0,
        rotation_speed: 80.0,
        projectile_speed: 2.7,
        cost: 210,
    },
    // 5
    StaticCannonData {
        range: 2.3,
        damage: 68.0,
        explosion_radius: 1.35,
        attack_speed: 1.1,
        rotation_speed: 80.0,
        projectile_speed: 2.9,
        cost: 300,
    },
    // 6
    StaticCannonData {
        range: 2.45,
        damage: 93.7,
        explosion_radius: 1.5,
        attack_speed: 1.1,
        rotation_speed: 90.0,
        projectile_speed: 3.0,
        cost: 420,
    },
    // 7
    StaticCannonData {
        range: 2.6,
        damage: 123.0,
        explosion_radius: 1.65,
        attack_speed: 1.25,
        rotation_speed: 100.0,
        projectile_speed: 3.1,
        cost: 850,
    },
    // 8
    StaticCannonData {
        range: 2.9,
        damage: 155.0,
        explosion_radius: 1.8,
        attack_speed: 1.35,
        rotation_speed: 100.0,
        projectile_speed: 3.2,
        cost: 1200,
    },
    // 9
    StaticCannonData {
        range: 3.2,
        damage: 204.0,
        explosion_radius: 1.95,
        attack_speed: 1.35,
        rotation_speed: 110.0,
        projectile_speed: 3.2,
        cost: 1950,
    },
    // 10
    StaticCannonData {
        range: 3.2,
        damage: 246.0,
        explosion_radius: 2.1,
        attack_speed: 1.4,
        rotation_speed: 120.0,
        projectile_speed: 3.2,
        cost: 3000,
    },
];

#[derive(Copy, Clone)]
pub struct DynamicCannonData {
    pub rotation: f32, // orientation/angle in RAD
    pub target: RecycledListRef,
}

impl FollowsTarget for DynamicCannonData {
    fn get_target(&self) -> RecycledListRef {
        self.target
    }

    fn set_target(&mut self, target: RecycledListRef) {
        self.target = target;
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn set_rotation(&mut self, new_rotation: f32) {
        self.rotation = new_rotation;
    }

    fn get_rotation_speed(&self, level: u32) -> f32 {
        CANNON[level as usize].rotation_speed
    }

    fn get_range(&self, level: u32) -> f32 {
        CANNON[level as usize].range
    }

    fn blast(&mut self, general_data: &mut GeneralData, state: &mut State, is_in_aim: bool) {
        let turret_data = &CANNON[general_data.level as usize];
        if is_in_aim
            && state.tick > general_data.last_shot + (60.0 / turret_data.attack_speed) as u32
        {
            // the turret position is the start of the barrel, where particles are emitted
            let x = (general_data.pos.x as f32 + 0.5) * state.cell_length
                + state.cell_length / 2.0 * self.rotation.cos();
            let y = (general_data.pos.y as f32 + 0.5) * state.cell_length
                + state.cell_length / 2.0 * self.rotation.sin();
            let turret_pos = FloatPosition { x, y };

            general_data.last_shot = state.tick;
            state.particles.add(Particle {
                pos: turret_pos,
                target: self.target,
                damage: turret_data.damage,
                speed: turret_data.projectile_speed * state.cell_length / 60.0,
                explosion_radius: turret_data.explosion_radius,
            });
        }
    }
}

#[derive(Copy, Clone)]
pub struct StaticMultiData {
    pub range: f32, // tiles
    pub damage: f32,
    pub attack_speed: f32,     // attacks/s
    pub rotation_speed: f32,   // deg/s
    pub projectile_speed: f32, // tiles/s
    pub cost: u32,
}

pub const MULTI: [StaticMultiData; 11] = [
    // 0
    StaticMultiData {
        range: 3.0,
        damage: 5.5,
        attack_speed: 1.0,
        rotation_speed: 50.0,
        projectile_speed: 2.0,
        cost: 90,
    },
    // 1
    StaticMultiData {
        range: 3.2,
        damage: 7.2,
        attack_speed: 1.1,
        rotation_speed: 55.0,
        projectile_speed: 2.2,
        cost: 35,
    },
    // 2
    StaticMultiData {
        range: 3.3,
        damage: 9.6,
        attack_speed: 1.25,
        rotation_speed: 60.0,
        projectile_speed: 2.4,
        cost: 68,
    },
    // 3
    StaticMultiData {
        range: 3.45,
        damage: 13.0,
        attack_speed: 1.25,
        rotation_speed: 68.0,
        projectile_speed: 2.6,
        cost: 120,
    },
    // 4
    StaticMultiData {
        range: 3.45,
        damage: 17.0,
        attack_speed: 1.4,
        rotation_speed: 70.0,
        projectile_speed: 2.6,
        cost: 170,
    },
    // 5
    StaticMultiData {
        range: 3.65,
        damage: 21.1,
        attack_speed: 1.55,
        rotation_speed: 70.0,
        projectile_speed: 2.8,
        cost: 280,
    },
    // 6
    StaticMultiData {
        range: 3.85,
        damage: 29.2,
        attack_speed: 1.55,
        rotation_speed: 74.0,
        projectile_speed: 3.0,
        cost: 460,
    },
    // 7
    StaticMultiData {
        range: 4.1,
        damage: 40.0,
        attack_speed: 1.7,
        rotation_speed: 80.0,
        projectile_speed: 3.0,
        cost: 660,
    },
    // 8
    StaticMultiData {
        range: 4.1,
        damage: 51.2,
        attack_speed: 1.8,
        rotation_speed: 85.0,
        projectile_speed: 3.25,
        cost: 1150,
    },
    // 9
    StaticMultiData {
        range: 4.35,
        damage: 64.0,
        attack_speed: 1.8,
        rotation_speed: 90.0,
        projectile_speed: 3.4,
        cost: 1750,
    },
    // 10
    StaticMultiData {
        range: 4.5,
        damage: 84.3,
        attack_speed: 1.9,
        rotation_speed: 90.0,
        projectile_speed: 3.5,
        cost: 2650,
    },
];

#[derive(Copy, Clone)]
pub struct DynamicMultiData {
    pub rotation: f32, // orientation/angle in RAD
    pub target: RecycledListRef,
}

impl FollowsTarget for DynamicMultiData {
    fn get_target(&self) -> RecycledListRef {
        self.target
    }

    fn set_target(&mut self, target: RecycledListRef) {
        self.target = target;
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn set_rotation(&mut self, new_rotation: f32) {
        self.rotation = new_rotation;
    }

    fn get_rotation_speed(&self, level: u32) -> f32 {
        MULTI[level as usize].rotation_speed
    }

    fn get_range(&self, level: u32) -> f32 {
        MULTI[level as usize].range
    }

    fn blast(&mut self, general_data: &mut GeneralData, state: &mut State, is_in_aim: bool) {
        let turret_data = &MULTI[general_data.level as usize];
        if is_in_aim
            && state.tick > general_data.last_shot + (60.0 / turret_data.attack_speed) as u32
        {
            // the turret position is the start of the barrel, where particles are emitted
            let x = (general_data.pos.x as f32 + 0.5) * state.cell_length
                + state.cell_length / 2.0 * self.rotation.cos();
            let y = (general_data.pos.y as f32 + 0.5) * state.cell_length
                + state.cell_length / 2.0 * self.rotation.sin();
            let turret_pos = FloatPosition { x, y };
            let speed = turret_data.projectile_speed * state.cell_length / 60.0;
            let lifetime = (state.cell_length
                * ((turret_data.range - state.cell_length / 60.0) / speed))
                as u32;
            // self.target.clone().

            let target_creep_option = state.creeps.get_clone(self.target);
            if Option::is_none(&target_creep_option) {
                return;
            }

            let target_creep = target_creep_option.unwrap();

            fn rotate(pos: FloatPosition, angle: f32) -> FloatPosition {
                let angle_radiants = PI * angle / 180.0;
                let c = angle_radiants.cos();
                let s = angle_radiants.sin();
                return FloatPosition {
                    x: pos.x * c + pos.y * s,
                    y: pos.y * c - pos.x * s,
                };
            }

            fn normalize(pos: FloatPosition) -> FloatPosition {
                let c = (pos.x * pos.x + pos.y * pos.y).sqrt();
                FloatPosition {
                    x: pos.x / c,
                    y: pos.y / c,
                }
            }
            let direction = normalize(target_creep.pos - turret_pos);

            general_data.last_shot = state.tick;

            state.multi_particles.add(MultiParticle {
                pos: turret_pos,
                direction,
                damage: turret_data.damage,
                lifetime_in_ticks: lifetime,
                speed,
            });
            state.multi_particles.add(MultiParticle {
                pos: turret_pos,
                direction: rotate(direction, 30.0),
                damage: turret_data.damage,
                lifetime_in_ticks: lifetime,
                speed,
            });
            state.multi_particles.add(MultiParticle {
                pos: turret_pos,
                direction: rotate(direction, -30.0),
                damage: turret_data.damage,
                lifetime_in_ticks: lifetime,
                speed,
            });
        }
    }
}

#[derive(Copy, Clone)]
pub enum SpecificData {
    Basic(DynamicBasicData),
    Sniper(DynamicSniperData), // fixme: create SniperData and use here
    Cannon(DynamicCannonData),
    Multi(DynamicMultiData),
}

#[derive(Copy, Clone)]
pub struct GeneralData {
    pub pos: GridPosition,
    pub last_shot: u32,
    pub level: u32,
}

#[derive(Copy, Clone)]
pub struct Turret {
    pub general_data: GeneralData,
    pub specific_data: SpecificData,
}

fn find_nearest_creep(
    creeps: &RecycledList<Creep>,
    turret_pos: FloatPosition,
    turret_range: f32,
) -> Option<&RecycledListItem<Creep>> {
    return creeps
        .enumerate()
        .map(|creep_item| (distance(creep_item.data.pos, turret_pos), creep_item))
        .filter(|(d, _item_ref)| *d < turret_range)
        .min_by_key(|(d, _item_ref)| (*d * 100.0) as i32)
        .map(|x| x.1);
}

pub trait FollowsTarget {
    fn get_target(&self) -> RecycledListRef;
    fn set_target(&mut self, target: RecycledListRef);
    fn get_rotation(&self) -> f32;
    fn set_rotation(&mut self, new_rotation: f32);
    fn get_rotation_speed(&self, level: u32) -> f32;
    fn get_range(&self, level: u32) -> f32;
    fn blast(&mut self, general_data: &mut GeneralData, state: &mut State, is_in_aim: bool);
}

pub fn update_tower(
    general_data: &mut GeneralData,
    specific: &mut impl FollowsTarget,
    state: &mut State,
) {
    let tower_pos = to_creep_position(general_data.pos, state.cell_length);

    // find target
    let target = state.creeps.get(specific.get_target());
    let level = general_data.level;
    if target.is_none() {
        match find_nearest_creep(
            &state.creeps,
            tower_pos,
            specific.get_range(level) * state.cell_length,
        ) {
            Some(creep) => {
                specific.set_target(creep.item_ref);
                return update_tower(general_data, specific, state);
            }
            None => return,
        }
    }

    let target_creep = target.unwrap();
    let creep_distance = distance(tower_pos, target_creep.pos);

    if creep_distance > specific.get_range(level) * state.cell_length {
        specific.set_target(RecycledListRef::null_ref());
        return update_tower(general_data, specific, state);
    }

    // rotate towards target
    let diff = target_creep.pos - tower_pos;

    let mut rotation_diff = f32::atan2(diff.y, diff.x) - specific.get_rotation();
    if rotation_diff > PI {
        rotation_diff -= TAU;
    }
    if rotation_diff < -PI {
        rotation_diff += TAU;
    }

    specific.set_rotation(
        specific.get_rotation()
            + rotation_diff.signum()
                * f32::min(
                    specific.get_rotation_speed(level).to_radians() / 60.0,
                    f32::abs(rotation_diff),
                ),
    );

    specific.blast(general_data, state, rotation_diff.abs() < 0.01);
}

impl Turret {
    pub fn tick(&mut self, state: &mut State) {
        let general_data = &mut self.general_data;

        match &mut self.specific_data {
            SpecificData::Basic(specific_data) => update_tower(general_data, specific_data, state),
            SpecificData::Sniper(specific_data) => update_tower(general_data, specific_data, state),
            SpecificData::Cannon(specific_data) => update_tower(general_data, specific_data, state),
            SpecificData::Multi(specific_data) => update_tower(general_data, specific_data, state),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Particle {
    pub pos: FloatPosition,
    // todo: remove "pub". should not leave api. this reference should not be needed for drawing. passing references
    // through api seems odd / hard to do in rust?
    pub target: RecycledListRef,
    pub damage: f32,
    pub speed: f32, // pixel per tick
    pub explosion_radius: f32,
}

pub trait ParticleWithLifetime {
    fn lifetime_in_ticks(&self) -> u32;
    fn decrement_lifetime(&mut self);
}

#[derive(Clone, Copy)]
pub struct SniperParticle {
    pub start_pos: FloatPosition,
    pub target_pos: FloatPosition,
    pub lifetime_in_ticks: u32, // delete at 0
}

impl ParticleWithLifetime for SniperParticle {
    fn lifetime_in_ticks(&self) -> u32 {
        self.lifetime_in_ticks
    }

    fn decrement_lifetime(&mut self) {
        self.lifetime_in_ticks -= 1
    }
}

#[derive(Clone, Copy)]
pub struct CannonParticle {
    pub pos: FloatPosition,
    pub explosion_radius: f32,
    pub lifetime_in_ticks: u32, // delete at 0
}

impl ParticleWithLifetime for CannonParticle {
    fn lifetime_in_ticks(&self) -> u32 {
        self.lifetime_in_ticks
    }

    fn decrement_lifetime(&mut self) {
        self.lifetime_in_ticks -= 1
    }
}

#[derive(Clone, Copy)]
pub struct MultiParticle {
    pub pos: FloatPosition,
    pub direction: FloatPosition,
    pub damage: f32,
    pub speed: f32,             // pixel per tick
    pub lifetime_in_ticks: u32, // delete at 0
}

impl ParticleWithLifetime for MultiParticle {
    fn lifetime_in_ticks(&self) -> u32 {
        self.lifetime_in_ticks
    }

    fn decrement_lifetime(&mut self) {
        self.lifetime_in_ticks -= 1
    }
}
