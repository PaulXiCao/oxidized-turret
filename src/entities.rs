use wasm_bindgen::prelude::*;

use crate::{
    recycled_list::RecycledListRef,
    utils::{FloatPosition, GridPosition},
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

pub const BASIC: [Basic; 1] = [Basic{
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
    pub specific_data: SpecificData
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Particle {
    pub pos: FloatPosition,
    // todo: remove "pub". should not leave api. this reference should not be needed for drawing. passing references
    // through api seems odd / hard to do in rust?
    pub target: RecycledListRef,
}
