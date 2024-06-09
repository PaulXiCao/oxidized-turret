use crate::entities::{Creep, Particle};
use crate::recycled_list::RecycledListRef;
use crate::utils::{to_float_position, FloatPosition};
use crate::{
    DynamicBasicData, DynamicSniperData, FollowsTarget, GamePhase, SpecificData, State, Turret,
    BASIC, SNIPER,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct ExternalTurret {
    pub pos: FloatPosition,
    pub rotation: f32, // orientation/angle in RAD
    pub range: f32,
    pub kind: i32,
}

pub fn to_external_turret(turret: &Turret, state: &State) -> ExternalTurret {
    ExternalTurret {
        pos: to_float_position(turret.general_data.pos, state.cell_length),
        rotation: match &turret.specific_data {
            SpecificData::Basic(d) => d.rotation,
            SpecificData::Sniper(d) => d.rotation,
        },
        range: match &turret.specific_data {
            SpecificData::Basic(d) => d.get_range(turret.general_data.level) * state.cell_length,
            SpecificData::Sniper(d) => d.get_range(turret.general_data.level) * state.cell_length,
        },
        kind: match &turret.specific_data {
            SpecificData::Basic(_) => 0,
            SpecificData::Sniper(_) => 1,
        },
    }
}

pub trait HasStats {
    fn stats(&self, level: u32) -> Vec<Stat>;
}

impl HasStats for DynamicBasicData {
    fn stats(&self, level: u32) -> Vec<Stat> {
        if level as usize >= BASIC.len() {
            return vec![];
        }

        vec![
            Stat {
                key: String::from("Level"),
                value: (level + 1) as f32,
                unit: String::from(""),
            },
            Stat {
                key: String::from("Attack Speed"),
                value: BASIC[level as usize].attack_speed,
                unit: String::from("/s"),
            },
            Stat {
                key: String::from("Range"),
                value: BASIC[level as usize].range,
                unit: String::from("tiles"),
            },
        ]
    }
}

impl HasStats for DynamicSniperData {
    fn stats(&self, level: u32) -> Vec<Stat> {
        if level as usize >= SNIPER.len() {
            return vec![];
        }

        vec![
            Stat {
                key: String::from("Level"),
                value: (level + 1) as f32,
                unit: String::from(""),
            },
            Stat {
                key: String::from("Attack Speed"),
                value: SNIPER[level as usize].attack_speed,
                unit: String::from("/s"),
            },
            Stat {
                key: String::from("Range"),
                value: SNIPER[level as usize].range,
                unit: String::from("tiles"),
            },
        ]
    }
}

pub fn to_external_turret_with_stats(turret: &Turret, state: &State) -> ExternalTurretWithStats {
    ExternalTurretWithStats {
        turret: to_external_turret(turret, state),
        stats: match turret.specific_data {
            SpecificData::Basic(d) => d.stats(turret.general_data.level),
            SpecificData::Sniper(d) => d.stats(turret.general_data.level),
        },
        next_stats: match turret.specific_data {
            SpecificData::Basic(d) => d.stats(turret.general_data.level + 1),
            SpecificData::Sniper(d) => d.stats(turret.general_data.level + 1),
        },
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct Stat {
    pub key: String,
    pub value: f32,
    pub unit: String,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct ExternalTurretWithStats {
    pub turret: ExternalTurret,
    pub stats: Vec<Stat>,
    pub next_stats: Vec<Stat>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct TurretRef {
    pub data: ExternalTurretWithStats,
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
    pub creep_goals: Vec<FloatPosition>,
    pub creep_path: Vec<FloatPosition>,
    pub turrets: Vec<ExternalTurret>,
    pub creeps: Vec<Creep>,
    pub particles: Vec<Particle>,
    pub cell_length: f32,
    pub health: u32,
    pub game_result: GameResult,
    pub current_level: u32,
    pub gold: u32,
    pub phase: GamePhase,
}
