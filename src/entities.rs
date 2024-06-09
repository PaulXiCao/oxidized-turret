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

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Creep {
    pub pos: FloatPosition,
    pub health: f32,
    pub max_health: f32,
    pub walking: WalkingProgress,
    pub speed: u32, // no. of ticks to walk one grid cell, lower is faster
    pub gold: u32,
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

pub const BASIC: [StaticBasicData; 2] = [
    StaticBasicData {
        range: 2.0,
        damage: 7.5,
        attack_speed: 1.25,
        rotation_speed: 90.0,
        projectile_speed: 2.8,
        damage_multiplier: 100.0,
        cost: 50,
    },
    StaticBasicData {
        range: 2.2,
        damage: 9.4,
        attack_speed: 1.4,
        rotation_speed: 90.0,
        projectile_speed: 3.0,
        damage_multiplier: 100.0,
        cost: 20,
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
                target: self.target.clone(),
                damage: turret_data.damage * turret_data.damage_multiplier / 100.0,
                speed: turret_data.projectile_speed * state.cell_length / 60.0,
            });
        }
    }
}

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

pub const SNIPER: [StaticSniperData; 1] = [StaticSniperData {
    range: 4.5,
    damage: 46.0,
    attack_speed: 0.28,
    rotation_speed: 50.0,
    aiming_speed: 90.0,
    crit_chance: 0.15,
    crit_multiplier: 1.5,
    cost: 80,
}];

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
            mut_target_creep.health -= turret_data.damage;
            if mut_target_creep.health <= 0.0 {
                state.gold += mut_target_creep.gold;
                state.creeps.remove(self.target);
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum SpecificData {
    Basic(DynamicBasicData),
    Sniper(DynamicSniperData), // fixme: create SniperData and use here
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
        .map_or(None, |x| Some(x.1));
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
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Particle {
    pub pos: FloatPosition,
    // todo: remove "pub". should not leave api. this reference should not be needed for drawing. passing references
    // through api seems odd / hard to do in rust?
    pub target: RecycledListRef,
    pub damage: f32,
    pub speed: f32, // pixel per tick
}
