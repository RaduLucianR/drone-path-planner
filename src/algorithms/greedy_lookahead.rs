use crate::map::GridMap;
use crate::utils::Position;
use std::time::{Duration, Instant};

fn in_bounds(nx: isize, ny: isize, bound: isize) -> bool {
    nx >= 0 && ny >= 0 && nx < bound && ny < bound
}

fn score_move(
    grid_map: &GridMap,
    x: usize,
    y: usize,
    depth: usize,
    directions: &[(isize, isize)],
) -> i32 {
    if depth == 0 {
        return 0;
    }

    let mut best = 0;

    for &(dx, dy) in directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if !((0..grid_map.get_size() as isize).contains(&nx)
            && (0..grid_map.get_size() as isize).contains(&ny))
        {
            continue;
        }

        let val = grid_map.get(nx as usize, ny as usize).unwrap_or(0);
        let future = score_move(grid_map, nx as usize, ny as usize, depth - 1, directions);
        best = best.max(val + future);
    }

    best
}

pub fn plan_path(
    grid_map: &mut GridMap,
    discrete_steps_count: u32,
    maximum_duration_millis: u64,
    start: Position,
    lookahead: usize,
) -> Vec<Position> {
    let mut path = vec![start];
    let mut x = start.x;
    let mut y = start.y;

    let directions: Vec<(isize, isize)> = (-1..=1)
        .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
        .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
        .collect();

    let start_time = Instant::now();
    let max_duration = Duration::from_millis(maximum_duration_millis);

    for _step in 0..discrete_steps_count {
        if start_time.elapsed() > max_duration {
            break;
        }

        grid_map.replenish();

        let mut best_move: Option<Position> = None;
        let mut best_value = i32::MIN;

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if !in_bounds(nx, ny, grid_map.get_size() as isize) {
                continue;
            }

            let nx_usize = nx as usize;
            let ny_usize = ny as usize;

            let immediate = grid_map.get(nx_usize, ny_usize).unwrap_or(0);
            let future = score_move(grid_map, nx_usize, ny_usize, lookahead - 1, &directions);
            let total = immediate + future;

            if total > best_value {
                best_value = total;
                best_move = Some(Position {
                    x: nx_usize,
                    y: ny_usize,
                });
            }
        }

        if let Some(Position { x: nx, y: ny }) = best_move {
            x = nx;
            y = ny;
            path.push(Position { x, y });
            grid_map.visit(x, y);
        } else {
            break;
        }
    }

    path
}
