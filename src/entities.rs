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
    pub health: u32,
    pub max_health: u32,
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
pub enum TurretKind {
    Basic,
}

#[derive(Copy, Clone)]
pub struct GeneralData {
    pub pos: GridPosition,
    pub last_shot: u32,
    pub level: u32,
    pub kind: TurretKind,
}

#[derive(Copy, Clone)]
pub struct Turret {
    pub general_data: GeneralData,
    pub specific_data: SpecificData,
}

impl Turret {
    pub fn tick(self: &mut Turret, state: &mut State) {
        let turret_rotation = match self.specific_data {
            SpecificData::Basic(d) => d.rotation,
        };
        let turret_data = match self.general_data.kind {
            TurretKind::Basic => BASIC[self.general_data.level as usize].clone(),
        };

        let x = (self.general_data.pos.x as f32 + 0.5) * state.cell_length
            + state.cell_length / 2.0 * turret_rotation.cos();
        let y = (self.general_data.pos.y as f32 + 0.5) * state.cell_length
            + state.cell_length / 2.0 * turret_rotation.sin();
        let turret_pos = FloatPosition { x, y };
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
        let target_creep_item = target_creep_item_option.unwrap();

        if target_creep_item.0 > turret_data.range * state.cell_length {
            return;
        }

        let target_creep_item = target_creep_item.1;
        let target_creep = target_creep_item.data;

        let dx = target_creep.pos.x - self.general_data.pos.x as f32 * state.cell_length;
        let dy = target_creep.pos.y - self.general_data.pos.y as f32 * state.cell_length;

        match &mut self.specific_data {
            SpecificData::Basic(d) => d.rotation = dy.atan2(dx),
        };
        if state.tick > self.general_data.last_shot + 60 {
            self.general_data.last_shot = state.tick;
            state.particles.add(Particle {
                pos: turret_pos,
                target: target_creep_item.item_ref.clone(),
            });
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
}
