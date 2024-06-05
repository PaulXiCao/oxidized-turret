mod entities;
mod external;
mod path;
mod recycled_list;
mod utils;

use entities::*;
use external::{ExternalState, ExternalTurret, GameResult, TurretRef};
use path::find_path;
use recycled_list::{RecycledList, RecycledListRef};
use utils::{
    distance, to_creep_position, to_float_position, to_grid_position, FloatPosition, GridPosition,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    state: State,
}

fn compute_creep_paths(state: &State) -> Option<Vec<Vec<FloatPosition>>> {
    let mut paths = vec![];

    let mut start = state.creep_spawn.clone();
    for goal in state.creep_goals.iter() {
        match find_path(
            start,
            *goal,
            state.board_dimension_x,
            state.board_dimension_y,
            &state.turrets,
        ) {
            Some(path) => paths.push(path),
            None => return None,
        }
        start = *goal;
    }
    Some(
        paths
            .iter()
            .map(|path| {
                path.0
                    .iter()
                    .map(|grid_pos| to_creep_position(*grid_pos, state.cell_length))
                    .collect()
            })
            .collect(),
    )
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        utils::set_panic_hook();

        let mut turrets: RecycledList<Turret> = RecycledList::new();

        let turret0 = Turret {
            pos: GridPosition { x: 2, y: 3 },
            rotation: 0.0,
            last_shot: 0,
            range: 100.0,
        };
        let turret1 = Turret {
            pos: GridPosition { x: 1, y: 9 },
            rotation: 0.0,
            last_shot: 0,
            range: 100.0,
        };

        turrets.add(turret0);
        turrets.add(turret1);

        let mut state = State {
            board_dimension_x: 20,
            board_dimension_y: 15,
            creep_spawn: GridPosition { x: 0, y: 9 },
            creep_goals: vec![GridPosition { x: 10, y: 5 }, GridPosition { x: 19, y: 9 }],
            creep_paths: vec![],
            last_spawn: 0,
            unspawned_creeps: 3,
            creep_count_per_level: 3,
            turrets,
            creeps: RecycledList::new(),
            particles: RecycledList::new(),
            cell_length: 30.0,
            health: 10,
            still_running: true,
            current_level: 1,
            max_level: 10,
            game_phase: GamePhase::Building,
            gold: 3,
            tick: 0,
        };
        state.creep_paths = compute_creep_paths(&state).unwrap();

        Game { state }
    }

    pub fn get_state(&self) -> ExternalState {
        let state = &self.state;

        let game_result = if state.still_running {
            GameResult::StillRunning
        } else if state.current_level > state.max_level {
            GameResult::PlayerWon
        } else {
            GameResult::CreepsWon
        };

        ExternalState {
            board_dimension_x: state.board_dimension_x as f32 * state.cell_length,
            board_dimension_y: state.board_dimension_y as f32 * state.cell_length,
            creep_spawn: to_float_position(state.creep_spawn, state.cell_length),
            creep_goals: state
                .creep_goals
                .iter()
                .map(|x| to_float_position(*x, state.cell_length))
                .collect(),
            creep_path: state.creep_paths.concat(),
            turrets: state
                .turrets
                .iter()
                .map(|x| ExternalTurret {
                    pos: to_float_position(x.pos, state.cell_length),
                    rotation: x.rotation,
                })
                .collect(),
            particles: state.particles.iter().map(|x| *x).collect(),
            creeps: state.creeps.iter().map(|x| *x).collect(),
            cell_length: state.cell_length,
            health: state.health,
            game_result,
            current_level: state.current_level,
        }
    }

    pub fn build_tower(&mut self, x: f32, y: f32) {
        match self.state.game_phase {
            GamePhase::Fighting => return,
            _ => (),
        }

        if self.state.gold == 0 {
            return;
        }

        if x < 0.0 || y < 0.0 {
            return;
        }

        let grid_pos = to_grid_position(FloatPosition { x, y }, self.state.cell_length);
        if grid_pos.x >= self.state.board_dimension_x || grid_pos.y >= self.state.board_dimension_y
        {
            return;
        }

        if self
            .state
            .turrets
            .iter()
            .find(|x| x.pos == grid_pos)
            .is_some()
        {
            return;
        }

        let tower_ref = self.state.turrets.add(Turret {
            pos: grid_pos,
            rotation: 0.0,
            last_shot: self.state.tick,
            range: 100.0,
        });

        match compute_creep_paths(&self.state) {
            Some(p) => {
                self.state.creep_paths = p;
                self.state.gold -= 1; // todo: implement variable tower costs
            }
            _ => self.state.turrets.remove(tower_ref),
        }
    }

    pub fn get_tower_at(self, x: f32, y: f32) -> Option<TurretRef> {
        let grid_pos = to_grid_position(FloatPosition { x, y }, self.state.cell_length);
        let value = self
            .state
            .turrets
            .enumerate()
            .find(|x| x.data.pos == grid_pos);
        match value {
            None => None,
            Some(x) => Some(TurretRef {
                turret: ExternalTurret {
                    pos: to_float_position(x.data.pos, self.state.cell_length),
                    rotation: x.data.rotation,
                },
                turret_ref: x.item_ref.clone(),
            }),
        }
    }

    pub fn update_state(&mut self) {
        if !self.state.still_running {
            return;
        }

        match self.state.game_phase {
            GamePhase::Building => {
                // todo: let player finish building phase without spending all gold
                if self.state.gold == 0 {
                    self.state.game_phase = GamePhase::Fighting;
                } else {
                    return;
                }
            }
            _ => (),
        }

        if (self.state.tick - self.state.last_spawn > 60) && (self.state.unspawned_creeps > 0) {
            self.state.last_spawn = self.state.tick;
            self.state.unspawned_creeps -= 1;
            self.state.creeps.add(Creep {
                pos: to_creep_position(self.state.creep_spawn, self.state.cell_length),
                health: 3,
                max_health: 10,
                walking: WalkingProgress {
                    current_goal: 0,
                    steps_taken: 0,
                    ticks_walked_since_previous_step: 0,
                },
                speed: 10,
            });
        }

        let mut creeps_to_remove: Vec<RecycledListRef> = vec![];
        for creep_item in self.state.creeps.enumerate_mut() {
            let creep = &mut creep_item.data;
            creep.walking.ticks_walked_since_previous_step += 1;
            if creep.walking.ticks_walked_since_previous_step >= creep.speed {
                // enough partial steps taken -> take one full step
                creep.walking.ticks_walked_since_previous_step = 0;
                creep.walking.steps_taken += 1;
                if creep.walking.steps_taken - 1
                    == self.state.creep_paths[creep.walking.current_goal as usize].len() as u32
                {
                    // reached one goal
                    creep.walking.current_goal += 1;
                    creep.walking.steps_taken = 0;
                    if creep.walking.current_goal as usize == self.state.creep_paths.len() {
                        // last goal reached
                        creeps_to_remove.push(creep_item.item_ref.clone());
                        self.state.health -= 1;
                        if self.state.health == 0 {
                            self.state.still_running = false;
                            return;
                        }
                    }
                }
            }

            {
                let t = creep.walking.ticks_walked_since_previous_step as f32 / creep.speed as f32;
                let path = &self.state.creep_paths[creep.walking.current_goal as usize];
                let a = path[creep.walking.steps_taken as usize];
                let b = path[creep.walking.steps_taken as usize + 1];
                let pos = a * (1.0 - t) + b * t;
                creep.pos = pos;
            }
        }
        for creep_to_remove in creeps_to_remove.iter() {
            self.state.creeps.remove(creep_to_remove.clone());
        }

        if (self.state.unspawned_creeps == 0) && self.state.creeps.is_empty() {
            self.state.current_level += 1;
            if self.state.current_level > self.state.max_level {
                self.state.still_running = false;
            }
            self.state.unspawned_creeps = self.state.creep_count_per_level;
            self.state.gold += 5; // todo: gold for finishing level should increase
            self.state.game_phase = GamePhase::Building;
            self.state.particles.clear();
            return;
        }

        for turret in self.state.turrets.iter_mut() {
            let x = (turret.pos.x as f32 + 0.5) * self.state.cell_length
                + self.state.cell_length / 2.0 * turret.rotation.cos();
            let y = (turret.pos.y as f32 + 0.5) * self.state.cell_length
                + self.state.cell_length / 2.0 * turret.rotation.sin();
            let turret_pos = FloatPosition { x, y };
            let mut distances = vec![];
            for creep_item in self.state.creeps.enumerate() {
                let d = distance(creep_item.data.pos, turret_pos);
                distances.push((d, creep_item));
            }
            let target_creep_item_option = distances
                .iter()
                .min_by_key(|(d, _item_ref)| (*d * 100.0) as i32);
            if target_creep_item_option.is_none() {
                break;
            }

            if target_creep_item_option.unwrap().0 > turret.range {
                continue;
            }

            let target_creep_item = target_creep_item_option.unwrap().1;
            let target_creep = target_creep_item.data;

            let dx = target_creep.pos.x - turret.pos.x as f32 * self.state.cell_length;
            let dy = target_creep.pos.y - turret.pos.y as f32 * self.state.cell_length;
            turret.rotation = dy.atan2(dx);
            if self.state.tick > turret.last_shot + 60 {
                turret.last_shot = self.state.tick;
                self.state.particles.add(Particle {
                    pos: turret_pos,
                    target: target_creep_item.item_ref.clone(),
                });
            }
        }

        let mut particles_to_remove: Vec<RecycledListRef> = vec![];

        for particle_item in self.state.particles.enumerate_mut() {
            let particle = &mut particle_item.data;
            let target_creep_option = self.state.creeps.get_mut(particle.target);
            if Option::is_none(&target_creep_option) {
                particles_to_remove.push(particle_item.item_ref);
                continue;
            }

            let target_creep = target_creep_option.unwrap();

            let d = distance(target_creep.pos, particle.pos);
            if d < 5.0 {
                particles_to_remove.push(particle_item.item_ref);
                if target_creep.health == 1 {
                    self.state.creeps.remove(particle.target.clone());
                    self.state.gold += 1; // todo: gold per killed creep depending on level?
                } else {
                    target_creep.health -= 1;
                }
            } else {
                let dx = target_creep.pos.x - particle.pos.x;
                let dy = target_creep.pos.y - particle.pos.y;
                particle.pos.x += (dx / d) * 5.0;
                particle.pos.y += (dy / d) * 5.0;
            }
        }

        for particle_to_remove in particles_to_remove.iter() {
            self.state.particles.remove(particle_to_remove.clone());
        }
        self.state.tick += 1;
    }
}

#[derive(Clone)]
enum GamePhase {
    Building,
    Fighting,
}

#[derive(Clone)]
pub struct State {
    // upper-left corner (0,0), lower-right corner (nx-1, nx-1)
    pub board_dimension_x: u32, // no. of grid points in x-direction
    pub board_dimension_y: u32, // no. of grid points in y-direction
    pub creep_spawn: GridPosition,
    pub creep_goals: Vec<GridPosition>,
    last_spawn: u32,
    unspawned_creeps: u32,
    creep_count_per_level: u32,
    pub creep_paths: Vec<Vec<FloatPosition>>,
    pub turrets: RecycledList<Turret>,
    pub creeps: RecycledList<Creep>,
    pub particles: RecycledList<Particle>,
    pub cell_length: f32,
    pub health: u32,
    pub still_running: bool,
    pub current_level: u32,
    pub max_level: u32,
    game_phase: GamePhase,
    gold: u32,

    tick: u32,
}
