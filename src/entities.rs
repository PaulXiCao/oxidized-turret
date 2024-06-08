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
pub struct Basic {
    pub range: f32, // tiles
    pub damage: f32,
    pub attack_speed: f32,      // attacks/s
    pub rotation_speed: f32,    // deg/s
    pub projectile_speed: f32,  // tiles/s
    pub damage_multiplier: f32, // 100% = normal damage
    pub cost: u32,
}

pub const BASIC: [Basic; 1] = [Basic {
    range: 2.0,
    damage: 7.5,
    attack_speed: 1.25,
    rotation_speed: 90.0,
    projectile_speed: 2.8,
    damage_multiplier: 100.0,
    cost: 50,
}];

// fixme: insert correct sniper data
pub const SNIPER: [Basic; 1] = [Basic {
    range: 2.0,
    damage: 7.5,
    attack_speed: 1.25,
    rotation_speed: 90.0,
    projectile_speed: 2.8,
    damage_multiplier: 100.0,
    cost: 10,
}];

#[derive(Copy, Clone)]
pub struct BasicData {
    pub rotation: f32, // orientation/angle in RAD
    pub target: RecycledListRef,
}

#[derive(Copy, Clone)]
pub enum SpecificData {
    Basic(BasicData),
    Sniper(BasicData), // fixme: create SniperData and use here
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

pub fn update_basic_tower(
    general_data: &mut GeneralData,
    specific: &mut BasicData,
    state: &mut State,
) {
    let turret_data = BASIC[general_data.level as usize];
    let tower_pos = to_creep_position(general_data.pos, state.cell_length);

    // find target
    let target = state.creeps.get(specific.target);
    if target.is_none() {
        match find_nearest_creep(
            &state.creeps,
            tower_pos,
            turret_data.range * state.cell_length,
        ) {
            Some(creep) => {
                specific.target = creep.item_ref;
                return update_basic_tower(general_data, specific, state);
            }
            None => return,
        }
    }

    let target_creep = target.unwrap();
    let creep_distance = distance(tower_pos, target_creep.pos);

    if creep_distance > turret_data.range * state.cell_length {
        specific.target = RecycledListRef::null_ref();
        return update_basic_tower(general_data, specific, state);
    }

    // rotate towards target
    let diff = target_creep.pos - tower_pos;

    let mut rotation_diff = diff.y.atan2(diff.x) - specific.rotation;
    if rotation_diff > PI {
        rotation_diff -= TAU;
    }
    if rotation_diff < -PI {
        rotation_diff += TAU;
    }

    specific.rotation += rotation_diff.signum()
        * (turret_data.rotation_speed.to_radians() / 60.0).min(rotation_diff.abs());

    // if ready for shooting, shoot
    if rotation_diff.abs() < 0.01
        && state.tick > general_data.last_shot + (60.0 / turret_data.attack_speed) as u32
    {
        // the turret position is the start of the barrel, where particles are emitted
        let x = (general_data.pos.x as f32 + 0.5) * state.cell_length
            + state.cell_length / 2.0 * specific.rotation.cos();
        let y = (general_data.pos.y as f32 + 0.5) * state.cell_length
            + state.cell_length / 2.0 * specific.rotation.sin();
        let turret_pos = FloatPosition { x, y };

        general_data.last_shot = state.tick;
        state.particles.add(Particle {
            pos: turret_pos,
            target: specific.target.clone(),
            damage: turret_data.damage * turret_data.damage_multiplier / 100.0,
            speed: turret_data.projectile_speed * state.cell_length / 60.0,
        });
    }
}

impl Turret {
    pub fn tick(&mut self, state: &mut State) {
        let general_data = &mut self.general_data;
        let specific_data = &mut self.specific_data;

        match specific_data {
            SpecificData::Basic(specific_data) => {
                update_basic_tower(general_data, specific_data, state)
            }
            SpecificData::Sniper(specific_data) => {
                update_basic_tower(general_data, specific_data, state) // fixme: implement update_sniper_tower
            }
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
