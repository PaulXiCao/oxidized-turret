use crate::{utils::FloatPosition, Creep, CreepKind, WalkingProgress};

#[derive(Clone)]
pub struct Spawn {
    pub quantity: u32,
    pub distance_in_ticks: u32,
    pub health: f32,
    pub speed: f32,
    pub bounty: u32, // bounty for the whole spawn. per creep this is bounty / quantity
    pub kind: CreepKind,
}

pub struct Spawner {
    pub pos: FloatPosition,
    pub spawn: Spawn,
    ticks: u32,
    last_spawn: u32,
    spawned_creeps: u32,
}

impl Spawner {
    pub fn new(pos: FloatPosition, spawn: Spawn) -> Spawner {
        Spawner {
            pos,
            spawn,
            ticks: 0,
            last_spawn: 0,
            spawned_creeps: 0,
        }
    }

    pub fn tick(&mut self) -> Option<Creep> {
        self.ticks += 1;

        if (self.ticks - self.last_spawn > self.spawn.distance_in_ticks) && (!self.is_finished()) {
            self.last_spawn = self.ticks;
            self.spawned_creeps += 1;

            let scaling = 1.0;

            return Some(Creep {
                pos: self.pos,
                health: self.spawn.health * scaling,
                max_health: self.spawn.health * scaling,
                walking: WalkingProgress {
                    current_goal: 0,
                    progress_made: 0.0,
                },
                speed: self.spawn.speed,
                gold: self.spawn.bounty,
                kind: self.spawn.kind,
                last_freeze_percent: 0.0,
                delta_speed: 0.0,
                slow_speed_accumulated: 0.0,
                max_freeze_percent: 0.0,
            });
        }
        None
    }

    pub fn is_finished(&self) -> bool {
        self.spawned_creeps >= self.spawn.quantity
    }

    pub fn reset(&mut self) {
        self.ticks = 0;
        self.last_spawn = 0;
        self.spawned_creeps = 0;
    }

    pub fn set_spawn(&mut self, spawn: Spawn) {
        self.spawn = spawn;
    }
}
