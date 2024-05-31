use crate::{utils, State};

use utils::GridPosition;

use pathfinding::prelude::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn successors(&self) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;
        vec![
            Pos(x + 1, y + 2),
            Pos(x + 1, y - 2),
            Pos(x - 1, y + 2),
            Pos(x - 1, y - 2),
            Pos(x + 2, y + 1),
            Pos(x + 2, y - 1),
            Pos(x - 2, y + 1),
            Pos(x - 2, y - 1),
        ]
        .into_iter()
        .map(|p| (p, 1))
        .collect()
    }
}

// pub fn find_path(state: &State) {
//     let GOAL = state.creep_goal;
//     let result = astar(
//         &Pos(1, 1),
//         |p| p.successors(),
//         |p| p.distance(&GOAL) / 3,
//         |p| *p == GOAL,
//     );
//     assert_eq!(result.expect("no path found").1, 4);
// }
