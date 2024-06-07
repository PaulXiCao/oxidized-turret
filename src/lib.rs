mod entities;
mod external;
mod path;
mod recycled_list;
mod utils;

use entities::*;
use external::{to_external_turret, ExternalState, ExternalTurret, GameResult, TurretRef};
use path::find_path;
use recycled_list::{RecycledList, RecycledListRef};
use utils::{
    distance, to_creep_position, to_float_position, to_grid_position, FloatPosition, GridPosition,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    state: State,
    turret_state: RecycledList<Turret>,
}

fn compute_creep_paths(game: &Game) -> Option<Vec<FloatPosition>> {
    let mut paths = vec![];

    let mut start = game.state.creep_spawn.clone();
    for goal in game.state.creep_goals.iter() {
        match find_path(
            start,
            *goal,
            game.state.board_dimension_x,
            game.state.board_dimension_y,
            &game.turret_state,
        ) {
            Some(path) => paths.push(path),
            None => return None,
        }
        start = *goal;
    }

    let mut result: Vec<Vec<FloatPosition>> = paths
        .iter()
        .map(|path| -> Vec<FloatPosition> {
            path.0
                .iter()
                .map(|grid_pos| to_creep_position(*grid_pos, game.state.cell_length))
                .collect()
        })
        .collect();

    // remove duplicate goals, due to the ending point of a subpath being equal
    // to the starting point of the next subpath
    for i in 0..result.len() - 1 {
        (&mut result)[i].pop();
    }

    Some(result.concat())
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        utils::set_panic_hook();

        let state = State {
            board_dimension_x: 20,
            board_dimension_y: 15,
            creep_spawn: GridPosition { x: 0, y: 9 },
            creep_goals: vec![GridPosition { x: 10, y: 5 }, GridPosition { x: 19, y: 9 }],
            creep_path: vec![],
            last_spawn: 0,
            unspawned_creeps: 10,
            creep_count_per_level: 10,
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

        let mut game = Game {
            state,
            turret_state: RecycledList::new(),
        };
        game.state.creep_path = compute_creep_paths(&game).unwrap();

        game
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
            creep_path: state.creep_path.clone(),
            turrets: self
                .turret_state
                .iter()
                .map(|x| to_external_turret(x, state))
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
            .turret_state
            .iter()
            .find(|x| x.general_data.pos == grid_pos)
            .is_some()
        {
            return;
        }

        let tower_ref = self.turret_state.add(Turret {
            general_data: GeneralData {
                pos: grid_pos,
                last_shot: self.state.tick,
                level: 0,
            },
            specific_data: SpecificData::Basic(BasicData { rotation: 0.0 }),
        });

        match compute_creep_paths(self) {
            Some(p) => {
                self.state.creep_path = p;
                self.state.gold -= 1; // todo: implement variable tower costs
            }
            _ => self.turret_state.remove(tower_ref),
        }
    }

    pub fn get_tower_at(self, x: f32, y: f32) -> Option<TurretRef> {
        let grid_pos = to_grid_position(FloatPosition { x, y }, self.state.cell_length);
        let value = self
            .turret_state
            .enumerate()
            .find(|x| x.data.general_data.pos == grid_pos);
        match value {
            None => None,
            Some(x) => Some(TurretRef {
                turret: ExternalTurret {
                    pos: to_float_position(x.data.general_data.pos, self.state.cell_length),
                    rotation: match &x.data.specific_data {
                        SpecificData::Basic(d) => d.rotation,
                    },
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
                health: 34.0,
                max_health: 34.0,
                walking: WalkingProgress {
                    current_goal: 0,
                    steps_taken: 0,
                },
                speed: 10,
            });
        }

        let mut creeps_to_remove: Vec<RecycledListRef> = vec![];
        for creep_item in self.state.creeps.enumerate_mut() {
            let creep = &mut creep_item.data;
            creep.walking.steps_taken += 1;
            if creep.walking.steps_taken >= creep.speed {
                creep.walking.current_goal += 1;
                creep.walking.steps_taken = 0;
            }
            if creep.walking.current_goal == self.state.creep_path.len() as u32 - 1 {
                creeps_to_remove.push(creep_item.item_ref.clone());
                self.state.health -= 1;
                if self.state.health == 0 {
                    self.state.still_running = false;
                    return;
                }
                continue;
            }

            {
                let t = creep.walking.steps_taken as f32 / creep.speed as f32;
                let path = &self.state.creep_path;
                let a = path[creep.walking.current_goal as usize];
                let b = path[creep.walking.current_goal as usize + 1];
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

        for turret in self.turret_state.iter_mut() {
            turret.tick(&mut self.state);
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
                target_creep.health -= particle.damage;
                if target_creep.health <= 0.0 {
                    self.state.creeps.remove(particle.target.clone());
                    self.state.gold += 1; // todo: gold per killed creep depending on level?
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
    pub creep_path: Vec<FloatPosition>,
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
