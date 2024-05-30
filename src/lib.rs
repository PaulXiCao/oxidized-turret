mod utils;

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
        let creep0 = Creep {
            x: 0.0,
            y: 200.0,
            health: 10,
            maxHealth: 10,
        };

        Game {
            state: State {
                board_dimension_x: 600,
                board_dimension_y: 400,
                turrets: vec![turret0, turret1],
                creeps: vec![creep0],
                particles: vec![],
                tick: 0,
            },
            // time: Instant::now(),
        }
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }

    pub fn update_state(&mut self) {
        for creep in self.state.creeps.iter_mut() {
            creep.x += 1.0;
            let xMax = self.state.board_dimension_x as f32;
            if creep.x > xMax {
                creep.x = xMax - creep.x;
            }
        }

        for turret in self.state.turrets.iter_mut() {
            let target_creep = self.state.creeps.get(0).unwrap();
            let dx = target_creep.x - turret.x as f32;
            let dy = target_creep.y - turret.y as f32;
            turret.rotation = dy.atan2(dx);
            if self.state.tick > turret.last_shot + 60 {
                turret.last_shot = self.state.tick;
                let x = turret.x as f32 + 15.0 * turret.rotation.cos();
                let y = turret.y as f32 + 15.0 * turret.rotation.sin();

                self.state.particles.push(Particle {
                    x: x,
                    y: y,
                    visible: true,
                    target: 0,
                });
            }
        }

        for particle in self.state.particles.iter_mut().filter(|p| p.visible) {
            let target_creep = self.state.creeps.get_mut(particle.target).unwrap();
            let dx = target_creep.x - particle.x;
            let dy = target_creep.y - particle.y;
            let d = (dx.powi(2) + dy.powi(2)).sqrt();
            if d < 5.0 {
                particle.visible = false;
                target_creep.health = match target_creep.health {
                    0 => 0,
                    _ => target_creep.health - 1,
                };
            } else {
                particle.x += (dx / d) * 5.0;
                particle.y += (dy / d) * 5.0;
            }
        }
        self.state.tick += 1;
    }
}
#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct State {
    // upper-left corner (0,0), lower-right corner (nx-1, nx-1)
    pub board_dimension_x: u32, // no. of grid points in x-direction
    pub board_dimension_y: u32, // no. of grid points in y-direction
    pub turrets: Vec<Turret>,
    pub creeps: Vec<Creep>,
    pub particles: Vec<Particle>,
    tick: u32,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct Creep {
    pub x: f32,
    pub y: f32,
    pub health: u32,
    pub maxHealth: u32,
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
#[derive(Clone, Debug)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub visible: bool,

    // todo: remove "pub". should not leave api. this reference should not be needed for drawing. passing references
    // through api seems odd / hard to do in rust?
    target: usize,
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
    x: u32,
    y: u32,
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
