use crate::recycled_list::RecycledList;
use crate::utils::GridPosition;
use crate::Turret;

use pathfinding::prelude::astar;

pub fn find_path(
    start: GridPosition,
    goal: GridPosition,
    nx: u32,
    ny: u32,
    turrets: &RecycledList<Turret>,
) -> Option<(Vec<GridPosition>, u32)> {
    astar(
        &start,
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
            if x + 1 < nx {
                successors.push(GridPosition { x: x + 1, y: y + 0 });
            }
            if y + 1 < ny {
                successors.push(GridPosition { x: x + 0, y: y + 1 });
            }

            for turret in turrets.iter() {
                if let Some(index) = successors.iter().position(|x| *x == turret.general_data.pos) {
                    successors.remove(index);
                }
            }

            successors.into_iter().map(|p| (p, 1)).collect()
        },
        |p| {
            let dx = goal.x as i32 - p.x as i32;
            let dy = goal.y as i32 - p.y as i32;
            (dx.abs() + dy.abs()) as u32
        },
        |p| *p == goal,
    )
}

#[test]
fn test_find_path() {

    let start = GridPosition { x: 0, y: 0 };
    let goal = GridPosition { x: 0, y: 10 };
    let nx = 20;
    let ny = 15;
    let turrets = RecycledList::new();

    let path = find_path(start, goal, nx, ny, &turrets);

    assert!(Option::is_some(&path));
}
