mod art;
mod entities;
mod external;
mod path;
mod recycled_list;
mod utils;

use art::Art;
use entities::*;
use external::{
    to_external_turret, to_external_turret_with_stats, ExternalState, GameResult, TurretRef,
};
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

    let mut start = game.state.creep_spawn;
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
            board_dimension_x: 40,
            board_dimension_y: 30,
            creep_spawn: GridPosition { x: 2, y: 0 },
            creep_goals: vec![
                GridPosition { x: 2, y: 15 },
                GridPosition { x: 37, y: 15 },
                GridPosition { x: 37, y: 2 },
                GridPosition { x: 20, y: 2 },
                GridPosition { x: 20, y: 27 },
            ],
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
            max_level: 1000,
            game_phase: GamePhase::Building,
            gold: 200,
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
            cell_length: state.cell_length,
            health: state.health,
            game_result,
            current_level: state.current_level,
            gold: state.gold,
            phase: state.game_phase.clone(),
        }
    }

    pub fn draw_state(&self, art: &Art, time: f32) {
        let state = &self.state;

        art.clear();
        art.drawMap(
            state.board_dimension_x as f32 * state.cell_length,
            state.board_dimension_y as f32 * state.cell_length,
        );

        art.startCreepPath(state.creep_path[0].x, state.creep_path[0].y, time);
        for line in &state.creep_path[1..] {
            art.drawCreepPathLine(line.x, line.y);
        }
        art.endCreepPath();

        let creep_spawn = to_float_position(state.creep_spawn, state.cell_length);
        art.drawCreepSpawn(creep_spawn.x, creep_spawn.y, state.cell_length);

        for goal in &state.creep_goals {
            let creep_goal = to_float_position(*goal, state.cell_length);
            art.drawCreepGoal(creep_goal.x, creep_goal.y, state.cell_length);
        }

        for turret in self.turret_state.iter() {
            let external_turret = to_external_turret(turret, state);
            art.drawTurret(
                external_turret.pos.x,
                external_turret.pos.y,
                external_turret.rotation,
                state.cell_length,
                external_turret.kind,
            )
        }

        for creep in state.creeps.iter() {
            art.drawCreep(creep.pos.x, creep.pos.y, creep.health / creep.max_health, 0);
        }

        for particle in state.particles.iter() {
            art.drawParticle(particle.pos.x, particle.pos.y);
        }
    }

    // fixme: create enum for kind instead of error-prone i32
    pub fn build_tower(&mut self, x: f32, y: f32, kind: i32) {
        if let GamePhase::Fighting = self.state.game_phase {
            return;
        }

        let cost = match kind {
            0 => BASIC[0].cost,
            1 => SNIPER[0].cost,
            _ => panic!("gotcha! tower kind not implemented!"),
        };

        if self.state.gold < cost {
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
            .any(|x| x.general_data.pos == grid_pos)
        {
            return;
        }

        let tower_ref = self.turret_state.add(Turret {
            general_data: GeneralData {
                pos: grid_pos,
                last_shot: self.state.tick,
                level: 0,
            },
            specific_data: match kind {
                0 => SpecificData::Basic(DynamicBasicData {
                    rotation: 0.0,
                    target: RecycledListRef::null_ref(),
                }),
                1 => SpecificData::Sniper(DynamicSniperData {
                    rotation: 0.0,
                    target: RecycledListRef::null_ref(),
                    aiming_ticks: 0,
                }),
                _ => panic!("gotcha! tower kind not implemented!"),
            },
        });

        match compute_creep_paths(self) {
            Some(p) => {
                self.state.creep_path = p;
                self.state.gold -= cost;
            }
            _ => self.turret_state.remove(tower_ref),
        }
    }

    pub fn get_tower_at(&self, x: f32, y: f32) -> Option<TurretRef> {
        let grid_pos = to_grid_position(FloatPosition { x, y }, self.state.cell_length);
        let value = self
            .turret_state
            .enumerate()
            .find(|x| x.data.general_data.pos == grid_pos);
        value.map(|x| TurretRef {
            data: to_external_turret_with_stats(&x.data, &self.state),
            turret_ref: x.item_ref,
        })
    }

    pub fn get_tower_by_ref(&self, turret_ref: RecycledListRef) -> Option<TurretRef> {
        self.turret_state.get(turret_ref).map(|turret| TurretRef {
            data: to_external_turret_with_stats(turret, &self.state),
            turret_ref,
        })
    }

    pub fn sell_tower(&mut self, turret_ref: RecycledListRef) {
        let tower_option = self.turret_state.get(turret_ref);
        if tower_option.is_none() {
            return;
        }

        let tower = tower_option.unwrap();
        match tower.specific_data {
            SpecificData::Basic(_) => self.state.gold += BASIC[0].cost,
            SpecificData::Sniper(_) => self.state.gold += SNIPER[0].cost,
        }
        self.turret_state.remove(turret_ref);
    }

    pub fn upgrade_tower(&mut self, turret_ref: RecycledListRef) {
        let tower_option = self.turret_state.get_mut(turret_ref);
        if tower_option.is_none() {
            return;
        }

        let tower = tower_option.unwrap();
        let max_level = match tower.specific_data {
            SpecificData::Basic(_) => BASIC.len(),
            SpecificData::Sniper(_) => SNIPER.len(),
        };
        let next_level = (tower.general_data.level + 1) as usize;
        if next_level >= max_level {
            return;
        }

        let cost = match tower.specific_data {
            SpecificData::Basic(_) => BASIC[next_level].cost,
            SpecificData::Sniper(_) => SNIPER[next_level].cost,
        };

        if self.state.gold < cost {
            return;
        }

        self.state.gold -= cost;
        tower.general_data.level += 1;
    }

    pub fn start_wave(&mut self) {
        if let GamePhase::Building = self.state.game_phase {
            self.state.game_phase = GamePhase::Fighting;
        }
    }

    pub fn update_state(&mut self) {
        if !self.state.still_running {
            return;
        }
        if let GamePhase::Building = self.state.game_phase {
            return;
        }

        if (self.state.tick - self.state.last_spawn > 120) && (self.state.unspawned_creeps > 0) {
            self.state.last_spawn = self.state.tick;
            self.state.unspawned_creeps -= 1;

            let scaling = 1.2_f32.powi(self.state.current_level as i32 - 1);

            self.state.creeps.add(Creep {
                pos: to_creep_position(self.state.creep_spawn, self.state.cell_length),
                health: 34.0 * scaling,
                max_health: 34.0 * scaling,
                walking: WalkingProgress {
                    current_goal: 0,
                    steps_taken: 0,
                },
                speed: 60,
                gold: 4 + self.state.current_level,
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
                creeps_to_remove.push(creep_item.item_ref);
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
            self.state.creeps.remove(*creep_to_remove);
        }

        if (self.state.unspawned_creeps == 0) && self.state.creeps.is_empty() {
            self.state.current_level += 1;
            self.state.creep_count_per_level += 1;

            if self.state.current_level > self.state.max_level {
                self.state.still_running = false;
            }
            self.state.unspawned_creeps = self.state.creep_count_per_level;
            // self.state.gold += 5; // todo: should we even have gold for finishing?
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
                    self.state.gold += target_creep.gold; // todo: gold per killed creep depending on level?
                    self.state.creeps.remove(particle.target);
                }
            } else {
                let dx = target_creep.pos.x - particle.pos.x;
                let dy = target_creep.pos.y - particle.pos.y;
                particle.pos.x += (dx / d) * particle.speed;
                particle.pos.y += (dy / d) * particle.speed;
            }
        }

        for particle_to_remove in particles_to_remove.iter() {
            self.state.particles.remove(*particle_to_remove);
        }
        self.state.tick += 1;
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
#[wasm_bindgen]
pub enum GamePhase {
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
