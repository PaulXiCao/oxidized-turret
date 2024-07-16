use crate::{spawn::Spawn, CreepKind};

fn create_normal(level: u32) -> Spawn {
    Spawn {
        quantity: 10 + level,
        distance_in_ticks: 60,
        health: 34.0 * 1.2_f32.powi(level as i32),
        speed: 1.0 / 60.0,
        bounty: 4 + level,
        kind: CreepKind::Normal,
    }
}

fn create_grouped(level: u32) -> Spawn {
    Spawn {
        quantity: (10 + level) * 3,
        distance_in_ticks: 20,
        health: (34.0 * 1.2_f32.powi(level as i32)) / 2.0,
        speed: 1.0 / 60.0,
        bounty: 2 + level / 2,
        kind: CreepKind::Grouped,
    }
}

fn create_speed(level: u32) -> Spawn {
    Spawn {
        quantity: 10 + level,
        distance_in_ticks: 60,
        health: 34.0 * 1.2_f32.powi(level as i32),
        speed: 1.4 / 60.0,
        bounty: 4 + level,
        kind: CreepKind::Speed,
    }
}

fn create_big(level: u32) -> Spawn {
    Spawn {
        quantity: 5 + level / 2,
        distance_in_ticks: 120,
        health: (34.0 * 1.2_f32.powi(level as i32)) * 2.5,
        speed: 0.9 / 60.0,
        bounty: (4 + level) * 2,
        kind: CreepKind::Big,
    }
}

pub fn create_level(current_level: u32) -> Spawn {
    match current_level % 4 {
        0 => create_normal(current_level),
        1 => create_grouped(current_level),
        2 => create_speed(current_level),
        3 => create_big(current_level),
        _ => todo!(),
    }
}
