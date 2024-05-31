mod recycled_list;
mod utils;

use recycled_list::{RecycledList, RecycledListRef};
use utils::{distance, FloatPosition};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    state: State,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        let turret0 = Turret {
            x: 50,
            y: 100,
            rotation: 0.0,
            last_shot: 0,
        };
        let turret1 = Turret {
            x: 300,
            y: 100,
            rotation: 0.0,
            last_shot: 0,
        };

        Game {
            state: State {
                board_dimension_x: 600,
                board_dimension_y: 400,
                creep_spawn: FloatPosition { x: 0.0, y: 200.0 },
                creep_goal: FloatPosition { x: 600.0, y: 200.0 },
                last_spawn: 0,
                turrets: vec![turret0, turret1],
                creeps: RecycledList::new(),
                particles: RecycledList::new(),
                tick: 0,
            },
        }
    }

    pub fn get_state(&self) -> ExternalState {
        let state = &self.state;
        ExternalState {
            board_dimension_x: state.board_dimension_x,
            board_dimension_y: state.board_dimension_y,
            turrets: state.turrets.clone(),
            particles: state.particles.iter().map(|x| *x).collect(),
            creeps: state.creeps.iter().map(|x| *x).collect(),
        }
    }

    pub fn update_state(&mut self) {
        if self.state.tick - self.state.last_spawn > 60 {
            self.state.last_spawn = self.state.tick;
            self.state.creeps.add(Creep {
                x: self.state.creep_spawn.x,
                y: self.state.creep_spawn.y,
                health: 4,
                max_health: 10,
            });
        }

        let mut creeps_to_remove: Vec<RecycledListRef> = vec![];
        for creep_item in self.state.creeps.enumerate_mut() {
            let creep = &mut creep_item.data;
            creep.x += 1.0;
            let d = distance(
                self.state.creep_goal,
                FloatPosition {
                    x: creep.x,
                    y: creep.y,
                },
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

            let dx = target_creep.x - turret.x as f32;
            let dy = target_creep.y - turret.y as f32;
            turret.rotation = dy.atan2(dx);
            if self.state.tick > turret.last_shot + 60 {
                turret.last_shot = self.state.tick;
                let x = turret.x as f32 + 15.0 * turret.rotation.cos();
                let y = turret.y as f32 + 15.0 * turret.rotation.sin();

                self.state.particles.add(Particle {
                    x,
                    y,
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
            let dx = target_creep.x - particle.x;
            let dy = target_creep.y - particle.y;
            let d = (dx.powi(2) + dy.powi(2)).sqrt();
            if d < 5.0 {
                particles_to_remove.push(particle_item.item_ref);
                if target_creep.health == 1 {
                    self.state.creeps.remove(particle.target.clone());
                } else {
                    target_creep.health -= 1;
                }
            } else {
                particle.x += (dx / d) * 5.0;
                particle.y += (dy / d) * 5.0;
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
    pub turrets: Vec<Turret>,
    pub creeps: Vec<Creep>,
    pub particles: Vec<Particle>,
}

#[derive(Clone)]
pub struct State {
    // upper-left corner (0,0), lower-right corner (nx-1, nx-1)
    pub board_dimension_x: u32, // no. of grid points in x-direction
    pub board_dimension_y: u32, // no. of grid points in y-direction
    pub creep_spawn: FloatPosition,
    pub creep_goal: FloatPosition,
    last_spawn: u32,
    pub turrets: Vec<Turret>,
    pub creeps: RecycledList<Creep>,
    pub particles: RecycledList<Particle>,
    tick: u32,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct Creep {
    pub x: f32,
    pub y: f32,
    pub health: u32,
    pub max_health: u32,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Turret {
    pub x: i32,
    pub y: i32,
    pub rotation: f32, // orientation/angle in RAD
    last_shot: u32,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Particle {
    pub x: f32,
    pub y: f32,

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
#[derive(Debug)]
pub struct Move {
    type_: MoveType,

    build_tower_data: Option<GridPosition>,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
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
        build_tower_data: Some(GridPosition { x: x, y: y }),
    }
}
