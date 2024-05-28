mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct State {
    pub board_dimension_x: u32,
    pub board_dimension_y: u32,
}

#[wasm_bindgen]
pub struct Game {
    state: State,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        Game {
            state: State {
                board_dimension_x: 600,
                board_dimension_y: 400,
            },
        }
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }

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
    fn build_tower(&mut self, pos: GridPosition) -> bool {
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
