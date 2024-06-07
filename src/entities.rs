use wasm_bindgen::prelude::*;

use crate::{
    recycled_list::RecycledListRef,
    utils::{distance, FloatPosition, GridPosition},
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

#[derive(Copy, Clone)]
pub struct BasicData {
    pub rotation: f32, // orientation/angle in RAD
}

#[derive(Copy, Clone)]
pub enum SpecificData {
    Basic(BasicData),
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

pub fn update_basic_tower(turret: &mut Turret, specific: &mut BasicData, state: &mut State) {
    let turret_data = BASIC[turret.general_data.level as usize];

    // the turret position is the start of the barrel, where particles are emitted
    let x = (turret.general_data.pos.x as f32 + 0.5) * state.cell_length
        + state.cell_length / 2.0 * specific.rotation.cos();
    let y = (turret.general_data.pos.y as f32 + 0.5) * state.cell_length
        + state.cell_length / 2.0 * specific.rotation.sin();
    let turret_pos = FloatPosition { x, y };

    // calculate current target
    let mut distances = vec![];
    for creep_item in state.creeps.enumerate() {
        let d = distance(creep_item.data.pos, turret_pos);
        distances.push((d, creep_item));
    }
    let target_creep_item_option = distances
        .iter()
        .min_by_key(|(d, _item_ref)| (*d * 100.0) as i32);
    if target_creep_item_option.is_none() {
        return;
    }

    // range check
    let target_creep_item = target_creep_item_option.unwrap();
    let creep_distance = target_creep_item.0;
    let target_creep_item = target_creep_item.1;

    if creep_distance > turret_data.range * state.cell_length {
        return;
    }

    // rotate towards target
    let target_creep = target_creep_item.data;

    let dx = target_creep.pos.x - turret.general_data.pos.x as f32 * state.cell_length;
    let dy = target_creep.pos.y - turret.general_data.pos.y as f32 * state.cell_length;

    specific.rotation = dy.atan2(dx);

    // if ready for shooting, shoot
    if state.tick > turret.general_data.last_shot + (60.0 / turret_data.attack_speed) as u32 {
        turret.general_data.last_shot = state.tick;
        state.particles.add(Particle {
            pos: turret_pos,
            target: target_creep_item.item_ref.clone(),
            damage: turret_data.damage * turret_data.damage_multiplier / 100.0,
            speed: turret_data.projectile_speed * state.cell_length / 60.0,
        });
    }
}

impl Turret {
    pub fn tick(self: &mut Turret, state: &mut State) {
        match self.specific_data {
            SpecificData::Basic(mut d) => update_basic_tower(self, &mut d, state),
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
