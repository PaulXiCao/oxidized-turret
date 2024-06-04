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

fn compute_creep_path(state: &State) -> Option<Vec<FloatPosition>> {
    let creep_path = find_path(state);
    if creep_path.is_some() {
        Some(
            creep_path
                .unwrap()
                .0
                .into_iter()
                .map(|x| to_creep_position(x, state.cell_length))
                .collect(),
        )
    } else {
        Option::None
    }
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
            creep_goal: GridPosition { x: 19, y: 9 },
            creep_path: vec![],
            last_spawn: 0,
            turrets,
            creeps: RecycledList::new(),
            particles: RecycledList::new(),
            cell_length: 30.0,
            health: 10,
            still_running: true,
            tick: 0,
        };
        state.creep_path = compute_creep_path(&state).unwrap();

        Game { state }
    }

    pub fn get_state(&self) -> ExternalState {
        let state = &self.state;

        let game_result = if state.still_running {
            GameResult::StillRunning
        } else if state.health > 0 {
            GameResult::PlayerWon
        } else {
            GameResult::CreepsWon
        };

        ExternalState {
            board_dimension_x: state.board_dimension_x as f32 * state.cell_length,
            board_dimension_y: state.board_dimension_y as f32 * state.cell_length,
            creep_spawn: to_float_position(state.creep_spawn, state.cell_length),
            creep_goal: to_float_position(state.creep_goal, state.cell_length),
            creep_path: state.creep_path.clone(),
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
        }
    }

    pub fn build_tower(&mut self, x: f32, y: f32) {
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

        match compute_creep_path(&self.state) {
            Some(p) => self.state.creep_path = p,
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

        if self.state.tick - self.state.last_spawn > 60 {
            self.state.last_spawn = self.state.tick;
            self.state.creeps.add(Creep {
                pos: to_creep_position(self.state.creep_spawn, self.state.cell_length),
                health: 10,
                max_health: 10,
                next_goal: 1,
                ticks_walked: 0,
                speed: 10,
            });
        }

        let mut creeps_to_remove: Vec<RecycledListRef> = vec![];
        for creep_item in self.state.creeps.enumerate_mut() {
            let creep = &mut creep_item.data;
            creep.ticks_walked += 1;
            if creep.ticks_walked >= creep.speed {
                creep.next_goal += 1;
                creep.ticks_walked = 0;
            }

            {
                let t = creep.ticks_walked as f32 / creep.speed as f32;
                let a = self.state.creep_path[creep.next_goal - 1];
                let b = self.state.creep_path[creep.next_goal];
                let pos = a * (1.0 - t) + b * t;
                creep.pos = pos;
            }

            let d = distance(
                to_creep_position(self.state.creep_goal, self.state.cell_length),
                creep.pos,
            );
            if d < 5.0 {
                creeps_to_remove.push(creep_item.item_ref.clone());
                self.state.health -= 1;
                if self.state.health == 0 {
                    self.state.still_running = false;
                    return;
                }
            }
        }
        for creep_to_remove in creeps_to_remove.iter() {
            self.state.creeps.remove(creep_to_remove.clone());
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
pub struct State {
    // upper-left corner (0,0), lower-right corner (nx-1, nx-1)
    pub board_dimension_x: u32, // no. of grid points in x-direction
    pub board_dimension_y: u32, // no. of grid points in y-direction
    pub creep_spawn: GridPosition,
    pub creep_goal: GridPosition,
    last_spawn: u32,
    pub creep_path: Vec<FloatPosition>,
    pub turrets: RecycledList<Turret>,
    pub creeps: RecycledList<Creep>,
    pub particles: RecycledList<Particle>,
    pub cell_length: f32,
    pub health: u32,
    pub still_running: bool,

    tick: u32,
}

//
// WIP
//

#[wasm_bindgen]
impl Game {
    pub fn get_possible_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        moves.push(create_forfeiting_move());
        moves.push(create_tower_building_move(2, 5));
        moves.push(create_tower_building_move(4, 5));
        moves
    }

    // @return did move apply cleanly? maybe not possible any longer..
    pub fn apply_move(&mut self, move_: Move) -> bool {
        match move_.type_ {
            MoveType::ForfeitGame => {
                self.forfeit();
                true
            }
            MoveType::BuildTower => {
                let _pos = match move_.build_tower_data {
                    None => return false,
                    Some(pos) => pos,
                };
                // self.build_tower(pos)
                true
            }
        }
    }

    fn forfeit(&mut self) {
        todo!()
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum MoveType {
    ForfeitGame,
    BuildTower,
}

#[wasm_bindgen]
pub struct Move {
    type_: MoveType,

    build_tower_data: Option<GridPosition>,
}

fn create_forfeiting_move() -> Move {
    Move {
        type_: MoveType::ForfeitGame,
        build_tower_data: Option::None,
    }
}

fn create_tower_building_move(x: u32, y: u32) -> Move {
    Move {
        type_: MoveType::BuildTower,
        build_tower_data: Some(GridPosition { x, y }),
    }
}
