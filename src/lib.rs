mod art;
mod entities;
mod external;
mod levels;
mod path;
mod recycled_list;
mod spawn;
mod utils;

use art::Art;
use entities::*;
use external::{
    to_external_turret, to_external_turret_with_stats, ExternalState, GameResult, TurretRef,
};
use levels::create_level;
use path::find_path;
use recycled_list::{RecycledList, RecycledListItem, RecycledListRef};
use spawn::Spawner;
use utils::{
    distance, to_creep_position, to_float_position, to_grid_position, FloatPosition, GridPosition,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    state: State,
    turret_state: RecycledList<Turret>,
    cannon_particles: RecycledList<CannonParticle>,
    spawner: Spawner,
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

        let creep_spawn = GridPosition { x: 2, y: 0 };
        let cell_length = 30.0;

        let state = State {
            board_dimension_x: 40,
            board_dimension_y: 30,
            creep_spawn,
            creep_goals: vec![
                GridPosition { x: 2, y: 15 },
                GridPosition { x: 37, y: 15 },
                GridPosition { x: 37, y: 2 },
                GridPosition { x: 20, y: 2 },
                GridPosition { x: 20, y: 27 },
            ],
            creep_path: vec![],
            creeps: RecycledList::new(),
            particles: RecycledList::new(),
            sniper_particles: RecycledList::new(),
            multi_particles: RecycledList::new(),
            cell_length,
            health: 10,
            still_running: true,
            current_level: 0,
            max_level: 50,
            game_phase: GamePhase::Building,
            gold: 200,
            tick: 0,
        };

        let mut game = Game {
            state,
            turret_state: RecycledList::new(),
            cannon_particles: RecycledList::new(),
            spawner: Spawner::new(to_creep_position(creep_spawn, cell_length), create_level(0)),
        };
        game.state.creep_path = compute_creep_paths(&game).unwrap();

        game
    }

    pub fn get_state(&self) -> ExternalState {
        let state = &self.state;

        let game_result = if state.still_running {
            GameResult::StillRunning
        } else if state.current_level >= state.max_level {
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

        for particle in self.cannon_particles.iter() {
            art.drawCannonParticle(
                particle.pos.x,
                particle.pos.y,
                particle.explosion_radius
                    * self.state.cell_length
                    * particle.lifetime_in_ticks as f32
                    / 20.0,
            );
        }

        for particle in self.state.sniper_particles.iter() {
            art.drawSniperParticle(
                particle.start_pos.x,
                particle.start_pos.y,
                particle.target_pos.x,
                particle.target_pos.y,
            );
        }

        for creep in state.creeps.iter() {
            art.drawCreep(
                creep.pos.x,
                creep.pos.y,
                creep.health / creep.max_health,
                creep.kind,
            );
        }

        for particle in state.particles.iter() {
            art.drawParticle(particle.pos.x, particle.pos.y);
        }

        for particle in state.multi_particles.iter() {
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
            2 => CANNON[0].cost,
            3 => MULTI[0].cost,
            4 => FREEZE[0].cost,
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
                2 => SpecificData::Cannon(DynamicCannonData {
                    rotation: 0.0,
                    target: RecycledListRef::null_ref(),
                }),
                3 => SpecificData::Multi(DynamicMultiData {
                    rotation: 0.0,
                    target: RecycledListRef::null_ref(),
                }),
                4 => SpecificData::Freeze(FREEZE[0]),
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

    pub fn sell_tower(&mut self, id: u32, index: usize) {
        if let GamePhase::Fighting = self.state.game_phase {
            return;
        }

        let turret_ref = RecycledListRef { id, index };

        let tower_option = self.turret_state.get(turret_ref);
        if tower_option.is_none() {
            return;
        }

        let tower = tower_option.unwrap();
        match tower.specific_data {
            SpecificData::Basic(_) => self.state.gold += BASIC[0].cost,
            SpecificData::Sniper(_) => self.state.gold += SNIPER[0].cost,
            SpecificData::Cannon(_) => self.state.gold += CANNON[0].cost,
            SpecificData::Multi(_) => self.state.gold += MULTI[0].cost,
            SpecificData::Freeze(_) => self.state.gold += FREEZE[0].cost,
        }
        self.turret_state.remove(turret_ref);

        // update creep path
        if let Some(p) = compute_creep_paths(self) {
            self.state.creep_path = p;
        }
    }

    pub fn upgrade_tower(&mut self, id: u32, index: usize) {
        if let GamePhase::Fighting = self.state.game_phase {
            return;
        }

        let turret_ref = RecycledListRef { id, index };
        let tower_option = self.turret_state.get_mut(turret_ref);
        if tower_option.is_none() {
            return;
        }

        let tower = tower_option.unwrap();
        let max_level = match tower.specific_data {
            SpecificData::Basic(_) => BASIC.len(),
            SpecificData::Sniper(_) => SNIPER.len(),
            SpecificData::Cannon(_) => CANNON.len(),
            SpecificData::Multi(_) => MULTI.len(),
            SpecificData::Freeze(_) => FREEZE.len(),
        };
        let next_level = (tower.general_data.level + 1) as usize;
        if next_level >= max_level {
            return;
        }

        let cost = match tower.specific_data {
            SpecificData::Basic(_) => BASIC[next_level].cost,
            SpecificData::Sniper(_) => SNIPER[next_level].cost,
            SpecificData::Cannon(_) => CANNON[next_level].cost,
            SpecificData::Multi(_) => MULTI[next_level].cost,
            SpecificData::Freeze(_) => FREEZE[next_level].cost,
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

        let creep_to_spawn = self.spawner.tick();

        if let Some(creep) = creep_to_spawn {
            self.state.creeps.add(creep);
        }

        // Reset freeze on all creepsa
        for creep in self.state.creeps.iter_mut() {
            creep.max_freeze_percent = 0.0;
            creep.delta_speed = -0.002 / 60.0; //creeps recover a fixed amount of 15 per second
        }
        for turret in self.turret_state.iter() {
            if let SpecificData::Freeze(turret_data) = turret.specific_data {
                for creep in self.state.creeps.iter_mut().filter(|the_creep| {
                    distance(
                        the_creep.pos,
                        turret.general_data.get_float_pos(self.state.cell_length),
                    ) <= turret_data.range * self.state.cell_length
                }) {
                    creep.max_freeze_percent =
                        f32::max(creep.max_freeze_percent, turret_data.freeze_percent);
                    // only reduce speed further if new freeze is more intense
                    if creep.last_freeze_percent < creep.max_freeze_percent {
                        creep.delta_speed = f32::max(creep.delta_speed, turret_data.freeze_speed);
                    }
                }
            }
            // match turret.specific_data {
            //     SpecificData::Freeze(turret_data) => {

            //     }
            //     _ => {}
            // }
        }
        // Set delta speed of all creeps
        for creep in self.state.creeps.iter_mut() {
            if creep.delta_speed == 0.0 {
                continue;
            }
            if creep.delta_speed < 0.0 {
                creep.slow_speed_accumulated =
                    f32::max(0.0, creep.slow_speed_accumulated + creep.delta_speed);
            } else {
                creep.slow_speed_accumulated = f32::min(
                    creep.speed * creep.max_freeze_percent,
                    creep.slow_speed_accumulated + creep.delta_speed,
                );
            }
            creep.last_freeze_percent = creep.slow_speed_accumulated / creep.speed;
        }

        let mut creeps_to_remove: Vec<RecycledListRef> = vec![];
        for creep_item in self.state.creeps.enumerate_mut() {
            let creep = &mut creep_item.data;
            creep.walking.progress_made += creep.speed - creep.slow_speed_accumulated;
            if creep.walking.progress_made >= 1.0 {
                creep.walking.current_goal += 1;
                creep.walking.progress_made = 0.0;
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
            // update creep position
            {
                let path = &self.state.creep_path;
                let a = path[creep.walking.current_goal as usize];
                let b = path[creep.walking.current_goal as usize + 1];
                let pos = a * (1.0 - creep.walking.progress_made) + b * creep.walking.progress_made;
                creep.pos = pos;
            }
        }
        for creep_to_remove in creeps_to_remove.iter() {
            self.state.creeps.remove(*creep_to_remove);
        }

        if self.spawner.is_finished() && self.state.creeps.is_empty() {
            self.state.current_level += 1;

            self.state.particles.clear();
            self.cannon_particles.clear();
            self.state.sniper_particles.clear();

            if self.state.current_level >= self.state.max_level {
                self.state.still_running = false;
                return;
            }

            self.spawner.reset();
            self.spawner
                .set_spawn(create_level(self.state.current_level));
            self.state.game_phase = GamePhase::Building;
            return;
        }

        for turret in self.turret_state.iter_mut() {
            turret.tick(&mut self.state);
        }

        let mut particles_to_remove: Vec<RecycledListRef> = vec![];
        let mut creeps_to_remove: Vec<RecycledListRef> = vec![];

        let cell_length = self.state.cell_length;
        let (particles, creeps, multi_particles, gold) = self.state.split_borrow();

        for particle_item in particles.enumerate_mut() {
            let particle = &mut particle_item.data;
            let target_creep_option = creeps.get_clone(particle.target);
            if Option::is_none(&target_creep_option) {
                particles_to_remove.push(particle_item.item_ref);
                continue;
            }

            let target_creep = target_creep_option.unwrap();

            let d = distance(target_creep.pos, particle.pos);
            if d < 5.0 {
                particles_to_remove.push(particle_item.item_ref);
                self.cannon_particles.add(CannonParticle {
                    pos: target_creep.pos,
                    explosion_radius: particle.explosion_radius,
                    lifetime_in_ticks: 20,
                });

                for creep_in_radius_item in creeps.enumerate_mut().filter(|creep| {
                    distance(creep.data.pos, target_creep.pos)
                        <= particle.explosion_radius * cell_length
                }) {
                    let creep_in_radius = &mut creep_in_radius_item.data;
                    creep_in_radius.health -= particle.damage;
                    if creep_in_radius.health <= 0.0 {
                        *gold += creep_in_radius.gold; // todo: gold per killed creep depending on level?
                        creeps_to_remove.push(creep_in_radius_item.item_ref);
                    }
                }
            } else {
                let dx = target_creep.pos.x - particle.pos.x;
                let dy = target_creep.pos.y - particle.pos.y;
                particle.pos.x += (dx / d) * particle.speed;
                particle.pos.y += (dy / d) * particle.speed;
            }
        }

        //Calculate damage for multi particles
        let mut multi_particles_to_remove: Vec<RecycledListRef> = vec![];
        for particle_item in multi_particles.enumerate_mut() {
            let particle = &mut particle_item.data;
            let mut best_distance: f32 = 10.0;
            let mut best_creep: Option<&mut RecycledListItem<Creep>> = None;
            for creep_in_radius_item in creeps.enumerate_mut() {
                let d = distance(creep_in_radius_item.data.pos, particle.pos);
                if d < best_distance {
                    best_creep = Some(creep_in_radius_item);
                    best_distance = d;
                }
            }
            if best_creep.is_some() {
                let creep_item = best_creep.unwrap();
                multi_particles_to_remove.push(particle_item.item_ref);
                let creep = creep_item;
                creep.data.health -= particle.damage;
                if creep.data.health <= 0.0 {
                    *gold += creep.data.gold; // todo: gold per killed creep depending on level?
                    creeps_to_remove.push(creep.item_ref);
                }
            }
        }
        for particle_to_remove in multi_particles_to_remove.iter() {
            multi_particles.remove(*particle_to_remove);
        }

        // Move multi particles
        for particle in multi_particles.iter_mut() {
            particle.pos += particle.direction * particle.speed;
        }

        // Cleanup
        for creep in creeps_to_remove.iter() {
            creeps.remove(*creep);
        }

        for particle_to_remove in particles_to_remove.iter() {
            self.state.particles.remove(*particle_to_remove);
        }

        update_particles_with_lifetime(&mut self.cannon_particles);
        update_particles_with_lifetime(&mut self.state.sniper_particles);
        update_particles_with_lifetime(&mut self.state.multi_particles);

        self.state.tick += 1;
    }
}

fn update_particles_with_lifetime<T: Clone + ParticleWithLifetime>(
    particles: &mut RecycledList<T>,
) {
    let mut particles_to_remove: Vec<RecycledListRef> = vec![];
    for particle_item in particles.enumerate_mut() {
        let particle = &mut particle_item.data;
        if particle.lifetime_in_ticks() == 1 {
            particles_to_remove.push(particle_item.item_ref);
        }
        particle.decrement_lifetime();
    }
    for particle_to_remove in particles_to_remove.iter() {
        particles.remove(*particle_to_remove);
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
    pub creep_path: Vec<FloatPosition>,
    pub creeps: RecycledList<Creep>,
    pub particles: RecycledList<Particle>,
    pub sniper_particles: RecycledList<SniperParticle>,
    pub multi_particles: RecycledList<MultiParticle>,
    pub cell_length: f32,
    pub health: u32,
    pub still_running: bool,
    pub current_level: u32,
    pub max_level: u32,
    game_phase: GamePhase,
    gold: u32,

    tick: u32,
}

impl State {
    fn split_borrow(
        &mut self,
    ) -> (
        &mut RecycledList<Particle>,
        &mut RecycledList<Creep>,
        &mut RecycledList<MultiParticle>,
        &mut u32,
    ) {
        (
            &mut self.particles,
            &mut self.creeps,
            &mut self.multi_particles,
            &mut self.gold,
        )
    }
}
