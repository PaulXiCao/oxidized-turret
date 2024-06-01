use crate::utils::{distance, to_float_position, GridPosition};
use crate::State;

use pathfinding::prelude::astar;

pub fn find_path(state: &State) -> Option<(Vec<GridPosition>, u32)> {
    let goal = state.creep_goal;
    astar(
        &state.creep_spawn,
        |p| -> Vec<(GridPosition, u32)> {
            let x = p.x;
            let y = p.y;

            let mut successors: Vec<GridPosition> = vec![];

            if x > 0 {
                successors.push(GridPosition { x: x - 1, y: y + 0 });
            }
            if y > 0 {
                successors.push(GridPosition { x: x + 0, y: y - 1 });
            }
            if x + 1 < state.board_dimension_x {
                successors.push(GridPosition { x: x + 1, y: y + 0 });
            }
            if y + 1 < state.board_dimension_y {
                successors.push(GridPosition { x: x + 0, y: y + 1 });
            }

            for turret in state.turrets.iter() {
                let index = successors.iter().position(|x| *x == turret.pos);
                if index.is_some() {
                    successors.remove(index.unwrap());
                }
            }

            successors.into_iter().map(|p| (p, 1)).collect()
        },
        |p| {
            distance(
                to_float_position(goal, state.cell_length),
                to_float_position(*p, state.cell_length),
            ) as u32
                / 2
        },
        |p| *p == goal,
    )
}

#[test]
fn test_find_path() {
    use crate::recycled_list::RecycledList;
    let state = State {
        board_dimension_x: 20,
        board_dimension_y: 15,
        creep_spawn: GridPosition { x: 0, y: 0 },
        creep_goal: GridPosition { x: 0, y: 1 },
        last_spawn: 0,
        turrets: RecycledList::new(),
        creeps: RecycledList::new(),
        particles: RecycledList::new(),
        cell_length: 30.0,
        tick: 0,
    };

    assert!(Option::is_some(&find_path(&state)));
}
