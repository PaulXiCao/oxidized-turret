use crate::recycled_list::RecycledListRef;
use crate::utils::{to_float_position, FloatPosition};
use crate::{
    DynamicBasicData, DynamicCannonData, DynamicSniperData, FollowsTarget, GamePhase, SpecificData,
    State, Turret, BASIC, CANNON, SNIPER,
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
            SpecificData::Cannon(d) => d.rotation,
        },
        range: match &turret.specific_data {
            SpecificData::Basic(d) => d.get_range(turret.general_data.level) * state.cell_length,
            SpecificData::Sniper(d) => d.get_range(turret.general_data.level) * state.cell_length,
            SpecificData::Cannon(d) => d.get_range(turret.general_data.level) * state.cell_length,
        },
        kind: match &turret.specific_data {
            SpecificData::Basic(_) => 0,
            SpecificData::Sniper(_) => 1,
            SpecificData::Cannon(_) => 2,
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
                key: String::from("Range"),
                value: BASIC[level as usize].range,
                unit: String::from("tiles"),
            },
            Stat {
                key: String::from("Damage"),
                value: BASIC[level as usize].damage,
                unit: String::from(""),
            },
            Stat {
                key: String::from("Attack speed"),
                value: BASIC[level as usize].attack_speed,
                unit: String::from("/s"),
            },
            Stat {
                key: String::from("Rotation speed"),
                value: BASIC[level as usize].rotation_speed,
                unit: String::from("deg/s"),
            },
            Stat {
                key: String::from("Projectile Speed"),
                value: BASIC[level as usize].projectile_speed,
                unit: String::from("tiles/s"),
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
                key: String::from("Range"),
                value: SNIPER[level as usize].range,
                unit: String::from("tiles"),
            },
            Stat {
                key: String::from("Damage"),
                value: SNIPER[level as usize].damage,
                unit: String::from(""),
            },
            Stat {
                key: String::from("Attack Speed"),
                value: SNIPER[level as usize].attack_speed,
                unit: String::from("/s"),
            },
            Stat {
                key: String::from("Rotation speed"),
                value: SNIPER[level as usize].rotation_speed,
                unit: String::from("deg/s"),
            },
            Stat {
                key: String::from("Aiming speed"),
                value: SNIPER[level as usize].aiming_speed,
                unit: String::from("%/s"),
            },
        ]
    }
}

impl HasStats for DynamicCannonData {
    fn stats(&self, level: u32) -> Vec<Stat> {
        if level as usize >= CANNON.len() {
            return vec![];
        }

        vec![
            Stat {
                key: String::from("Level"),
                value: (level + 1) as f32,
                unit: String::from(""),
            },
            Stat {
                key: String::from("Range"),
                value: CANNON[level as usize].range,
                unit: String::from("tiles"),
            },
            Stat {
                key: String::from("Damage"),
                value: CANNON[level as usize].damage,
                unit: String::from(""),
            },
            Stat {
                key: String::from("Attack Speed"),
                value: CANNON[level as usize].attack_speed,
                unit: String::from("/s"),
            },
            Stat {
                key: String::from("Rotation speed"),
                value: CANNON[level as usize].rotation_speed,
                unit: String::from("deg/s"),
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
            SpecificData::Cannon(d) => d.stats(turret.general_data.level),
        },
        next_stats: match turret.specific_data {
            SpecificData::Basic(d) => d.stats(turret.general_data.level + 1),
            SpecificData::Sniper(d) => d.stats(turret.general_data.level + 1),
            SpecificData::Cannon(d) => d.stats(turret.general_data.level + 1),
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
    pub cell_length: f32,
    pub health: u32,
    pub game_result: GameResult,
    pub current_level: u32,
    pub gold: u32,
    pub phase: GamePhase,
}
