mod path;
mod recycled_list;
mod utils;

use recycled_list::{RecycledList, RecycledListRef};
use utils::{distance, to_float_position, FloatPosition, GridPosition};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    state: State,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        let turret0 = Turret {
            pos: GridPosition { x: 2, y: 3 },
            rotation: 0.0,
            last_shot: 0,
        };
        let turret1 = Turret {
            pos: GridPosition { x: 10, y: 3 },
            rotation: 0.0,
            last_shot: 0,
        };

        Game {
            state: State {
                board_dimension_x: 20,
                board_dimension_y: 15,
                creep_spawn: GridPosition { x: 0, y: 9 },
                creep_goal: GridPosition { x: 19, y: 9 },
                last_spawn: 0,
                turrets: vec![turret0, turret1],
                creeps: RecycledList::new(),
                particles: RecycledList::new(),
                cell_length: 30.0,
                tick: 0,
            },
        }
    }

    pub fn get_state(&self) -> ExternalState {
        let state = &self.state;

        let mut creep_path: Vec<GridPosition> = vec![];
        for x in state.creep_spawn.x..state.creep_goal.x + 1 {
            creep_path.push(GridPosition {
                x,
                y: state.creep_spawn.y,
            });
        }

        ExternalState {
            board_dimension_x: state.board_dimension_x,
            board_dimension_y: state.board_dimension_y,
            creep_spawn: state.creep_spawn,
            creep_goal: state.creep_goal,
            creep_path,
            turrets: state.turrets.clone(),
            particles: state.particles.iter().map(|x| *x).collect(),
            creeps: state.creeps.iter().map(|x| *x).collect(),
            cell_length: state.cell_length,
        }
    }

    pub fn update_state(&mut self) {
        if self.state.tick - self.state.last_spawn > 60 {
            self.state.last_spawn = self.state.tick;
            self.state.creeps.add(Creep {
                pos: to_float_position(self.state.creep_spawn, self.state.cell_length),
                health: 4,
                max_health: 10,
            });
        }

        let mut creeps_to_remove: Vec<RecycledListRef> = vec![];
        for creep_item in self.state.creeps.enumerate_mut() {
            let creep = &mut creep_item.data;
            creep.pos.x += 1.0;
            let d = distance(
                to_float_position(self.state.creep_goal, self.state.cell_length),
                creep.pos,
            );
            if d < 5.0 {
                creeps_to_remove.push(creep_item.item_ref.clone());
            }
        }
        for creep_to_remove in creeps_to_remove.iter() {
            self.state.creeps.remove(creep_to_remove.clone());
        }

        for turret in self.state.turrets.iter_mut() {
            let target_creep_item_option = self.state.creeps.enumerate().next();
            if Option::is_none(&target_creep_item_option) {
                continue;
            }
            let target_creep_item = target_creep_item_option.unwrap();
            let target_creep = target_creep_item.data;

            let dx = target_creep.pos.x - turret.pos.x as f32 * self.state.cell_length;
            let dy = target_creep.pos.y - turret.pos.y as f32 * self.state.cell_length;
            turret.rotation = dy.atan2(dx);
            if self.state.tick > turret.last_shot + 60 {
                turret.last_shot = self.state.tick;
                let x = turret.pos.x as f32 * self.state.cell_length + 15.0 * turret.rotation.cos();
                let y = turret.pos.y as f32 * self.state.cell_length + 15.0 * turret.rotation.sin();

                self.state.particles.add(Particle {
                    pos: FloatPosition { x, y },
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
#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct ExternalState {
    // upper-left corner (0,0), lower-right corner (nx-1, nx-1)
    pub board_dimension_x: u32, // no. of grid points in x-direction
    pub board_dimension_y: u32, // no. of grid points in y-direction
    pub creep_spawn: GridPosition,
    pub creep_goal: GridPosition,
    pub creep_path: Vec<GridPosition>,
    pub turrets: Vec<Turret>,
    pub creeps: Vec<Creep>,
    pub particles: Vec<Particle>,
    pub cell_length: f32,
}

#[derive(Clone)]
pub struct State {
    // upper-left corner (0,0), lower-right corner (nx-1, nx-1)
    pub board_dimension_x: u32, // no. of grid points in x-direction
    pub board_dimension_y: u32, // no. of grid points in y-direction
    pub creep_spawn: GridPosition,
    pub creep_goal: GridPosition,
    last_spawn: u32,
    pub turrets: Vec<Turret>,
    pub creeps: RecycledList<Creep>,
    pub particles: RecycledList<Particle>,
    pub cell_length: f32,
    tick: u32,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Creep {
    pub pos: FloatPosition,
    pub health: u32,
    pub max_health: u32,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Turret {
    pub pos: GridPosition,
    pub rotation: f32, // orientation/angle in RAD
    last_shot: u32,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Particle {
    pub pos: FloatPosition,
    // todo: remove "pub". should not leave api. this reference should not be needed for drawing. passing references
    // through api seems odd / hard to do in rust?
    target: RecycledListRef,
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
                let pos = match move_.build_tower_data {
                    None => return false,
                    Some(pos) => pos,
                };
                self.build_tower(pos)
            }
        }
    }

    // @return tower building successfull?
    fn build_tower(&mut self, _pos: GridPosition) -> bool {
        todo!()
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
