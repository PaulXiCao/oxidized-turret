use crate::recycled_list::RecycledListRef;
use crate::utils::{to_float_position, FloatPosition};
use crate::{
    DynamicBasicData, DynamicCannonData, DynamicMultiData, DynamicSniperData, FollowsTarget,
    GamePhase, HasCost, SpecificData, State, StaticFreezeData, Turret, BASIC, CANNON, FREEZE,
    MULTI, SNIPER,
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
            SpecificData::Multi(d) => d.rotation,
            SpecificData::Freeze(_) => 0.0,
        },
        range: match &turret.specific_data {
            SpecificData::Basic(d) => d.get_range(turret.general_data.level) * state.cell_length,
            SpecificData::Sniper(d) => d.get_range(turret.general_data.level) * state.cell_length,
            SpecificData::Cannon(d) => d.get_range(turret.general_data.level) * state.cell_length,
            SpecificData::Multi(d) => d.get_range(turret.general_data.level) * state.cell_length,
            SpecificData::Freeze(d) => d.get_range(turret.general_data.level) * state.cell_length,
        },
        kind: match &turret.specific_data {
            SpecificData::Basic(_) => 0,
            SpecificData::Sniper(_) => 1,
            SpecificData::Cannon(_) => 2,
            SpecificData::Multi(_) => 3,
            SpecificData::Freeze(_) => 4,
        },
    }
}

pub trait HasStats {
    fn stats(&self, level: u32) -> Vec<Stat>;
}

fn get_cost<T: HasCost>(arr: &[T], level: usize) -> f32 {
    (arr[level].get_cost()
        + if level == 0 {
            0
        } else {
            let mut cost = 0;
            for val in arr.iter().take(level) {
                cost += val.get_cost();
            }
            cost
        }) as f32
}

impl HasStats for DynamicBasicData {
    fn stats(&self, level: u32) -> Vec<Stat> {
        let level = level as usize;
        if level >= BASIC.len() {
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
                value: BASIC[level].range,
                unit: String::from("tiles"),
            },
            Stat {
                key: String::from("Damage"),
                value: BASIC[level].damage,
                unit: String::from(""),
            },
            Stat {
                key: String::from("Attack speed"),
                value: BASIC[level].attack_speed,
                unit: String::from("/s"),
            },
            Stat {
                key: String::from("Rotation speed"),
                value: BASIC[level].rotation_speed,
                unit: String::from("deg/s"),
            },
            Stat {
                key: String::from("Projectile Speed"),
                value: BASIC[level].projectile_speed,
                unit: String::from("tiles/s"),
            },
            Stat {
                key: String::from("Cost"),
                value: get_cost(&BASIC, level),
                unit: String::from("gold"),
            },
        ]
    }
}

impl HasStats for DynamicMultiData {
    fn stats(&self, level: u32) -> Vec<Stat> {
        let level = level as usize;
        if level >= MULTI.len() {
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
                value: MULTI[level].range,
                unit: String::from("tiles"),
            },
            Stat {
                key: String::from("Damage"),
                value: MULTI[level].damage,
                unit: String::from(""),
            },
            Stat {
                key: String::from("Attack speed"),
                value: MULTI[level].attack_speed,
                unit: String::from("/s"),
            },
            Stat {
                key: String::from("Rotation speed"),
                value: MULTI[level].rotation_speed,
                unit: String::from("deg/s"),
            },
            Stat {
                key: String::from("Projectile Speed"),
                value: MULTI[level].projectile_speed,
                unit: String::from("tiles/s"),
            },
            Stat {
                key: String::from("Cost"),
                value: get_cost(&MULTI, level),
                unit: String::from("gold"),
            },
        ]
    }
}

impl HasStats for DynamicSniperData {
    fn stats(&self, level: u32) -> Vec<Stat> {
        let level = level as usize;
        if level >= SNIPER.len() {
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
                value: SNIPER[level].range,
                unit: String::from("tiles"),
            },
            Stat {
                key: String::from("Damage"),
                value: SNIPER[level].damage,
                unit: String::from(""),
            },
            Stat {
                key: String::from("Attack Speed"),
                value: SNIPER[level].attack_speed,
                unit: String::from("/s"),
            },
            Stat {
                key: String::from("Rotation speed"),
                value: SNIPER[level].rotation_speed,
                unit: String::from("deg/s"),
            },
            Stat {
                key: String::from("Aiming speed"),
                value: SNIPER[level].aiming_speed,
                unit: String::from("%/s"),
            },
            Stat {
                key: String::from("Cost"),
                value: get_cost(&SNIPER, level),
                unit: String::from("gold"),
            },
        ]
    }
}

impl HasStats for DynamicCannonData {
    fn stats(&self, level: u32) -> Vec<Stat> {
        let level = level as usize;
        if level >= CANNON.len() {
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
                value: CANNON[level].range,
                unit: String::from("tiles"),
            },
            Stat {
                key: String::from("Damage"),
                value: CANNON[level].damage,
                unit: String::from(""),
            },
            Stat {
                key: String::from("Attack Speed"),
                value: CANNON[level].attack_speed,
                unit: String::from("/s"),
            },
            Stat {
                key: String::from("Rotation speed"),
                value: CANNON[level].rotation_speed,
                unit: String::from("deg/s"),
            },
            Stat {
                key: String::from("Cost"),
                value: get_cost(&CANNON, level),
                unit: String::from("gold"),
            },
        ]
    }
}

impl HasStats for StaticFreezeData {
    fn stats(&self, level: u32) -> Vec<Stat> {
        let level = level as usize;
        if level >= FREEZE.len() {
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
                value: FREEZE[level].range,
                unit: String::from("tiles"),
            },
            Stat {
                key: String::from("Freeze Percent"),
                value: FREEZE[level].freeze_percent,
                unit: String::from(""),
            },
            Stat {
                key: String::from("Freeze  Speed"),
                value: FREEZE[level].freeze_speed,
                unit: String::from("/s"),
            },
            Stat {
                key: String::from("Cost"),
                value: get_cost(&FREEZE, level),
                unit: String::from("gold"),
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
            SpecificData::Multi(d) => d.stats(turret.general_data.level),
            SpecificData::Freeze(d) => d.stats(turret.general_data.level),
        },
        next_stats: match turret.specific_data {
            SpecificData::Basic(d) => d.stats(turret.general_data.level + 1),
            SpecificData::Sniper(d) => d.stats(turret.general_data.level + 1),
            SpecificData::Cannon(d) => d.stats(turret.general_data.level + 1),
            SpecificData::Multi(d) => d.stats(turret.general_data.level + 1),
            SpecificData::Freeze(d) => d.stats(turret.general_data.level + 1),
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
