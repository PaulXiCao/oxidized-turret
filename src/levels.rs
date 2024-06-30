use crate::spawn::Spawn;

fn create_normal(level: u32) -> Spawn {
    Spawn {
        quantity: 10 + level,
        distance_in_ticks: 60,
        health: 34.0 * 1.2_f32.powi(level as i32),
        speed: 60,
        bounty: 4 + level,
    }
}

fn create_grouped(level: u32) -> Spawn {
    Spawn {
        quantity: (10 + level) * 2,
        distance_in_ticks: 60,
        health: (34.0 * 1.2_f32.powi(level as i32)) / 2.0,
        speed: 60,
        bounty: 2 + level / 2,
    }
}

fn create_speed(level: u32) -> Spawn {
    Spawn {
        quantity: 10 + level,
        distance_in_ticks: 60,
        health: 34.0 * 1.2_f32.powi(level as i32),
        speed: 30,
        bounty: 4 + level,
    }
}

fn create_big(level: u32) -> Spawn {
    Spawn {
        quantity: 5 + level / 2,
        distance_in_ticks: 120,
        health: (34.0 * 1.2_f32.powi(level as i32)) * 2.0,
        speed: 60,
        bounty: (4 + level) * 2,
    }
}

pub fn create_levels(num_levels: u32) -> Vec<Spawn> {
    let mut levels = vec![];

    for current_level in 0..num_levels {
        levels.push(match current_level % 4 {
            0 => create_normal(current_level),
            1 => create_grouped(current_level),
            2 => create_speed(current_level),
            3 => create_big(current_level),
            _ => todo!(),
        });
    }

    levels
}
