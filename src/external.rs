use crate::entities::{Creep, Particle};
use crate::recycled_list::RecycledListRef;
use crate::utils::FloatPosition;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct ExternalTurret {
    pub pos: FloatPosition,
    pub rotation: f32, // orientation/angle in RAD
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TurretRef {
    pub turret: ExternalTurret,
    pub turret_ref: RecycledListRef,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameResult {
    StillRunning,
    CreepsWon,
    PlayerWon,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct ExternalState {
    // upper-left corner (0,0), lower-right corner (nx-1, nx-1)
    pub board_dimension_x: f32, // no. of grid points in x-direction
    pub board_dimension_y: f32, // no. of grid points in y-direction
    pub creep_spawn: FloatPosition,
    pub creep_goal: FloatPosition,
    pub creep_path: Vec<FloatPosition>,
    pub turrets: Vec<ExternalTurret>,
    pub creeps: Vec<Creep>,
    pub particles: Vec<Particle>,
    pub cell_length: f32,
    pub health: u32,
    pub game_result: GameResult,
    pub current_level: u32,
}