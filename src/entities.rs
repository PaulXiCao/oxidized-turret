use wasm_bindgen::prelude::*;

use crate::{
    recycled_list::RecycledListRef,
    utils::{FloatPosition, GridPosition},
};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Creep {
    pub pos: FloatPosition,
    pub health: u32,
    pub max_health: u32,
    pub next_goal: usize,
    pub ticks_walked: u32,
    pub speed: u32, // no. of ticks to walk one grid cell, lower is faster
}

#[derive(Clone)]
pub struct Turret {
    pub pos: GridPosition,
    pub rotation: f32, // orientation/angle in RAD
    pub last_shot: u32,
    pub range: f32,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Particle {
    pub pos: FloatPosition,
    // todo: remove "pub". should not leave api. this reference should not be needed for drawing. passing references
    // through api seems odd / hard to do in rust?
    pub target: RecycledListRef,
}
