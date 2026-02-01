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
) -> f32 {
    if depth == 0 {
        return 0.0;
    }

    let mut best: f32 = 0.0;

    for &(dx, dy) in directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if !((0..grid_map.get_size() as isize).contains(&nx)
            && (0..grid_map.get_size() as isize).contains(&ny))
        {
            continue;
        }

        let val = grid_map.get(nx as usize, ny as usize).unwrap_or(0.0);
        let future = score_move(grid_map, nx as usize, ny as usize, depth - 1, directions);
        best = best.max(val + future);
    }

    best
}

pub fn greedy_lookahead(
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
            log::warn!("Time limit exceeded, exiting...");
            break;
        }

        grid_map.replenish();

        let mut best_move: Option<Position> = None;
        let mut best_value = f32::MIN;

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if !in_bounds(nx, ny, grid_map.get_size() as isize) {
                continue;
            }

            let nx_usize = nx as usize;
            let ny_usize = ny as usize;

            let immediate = grid_map.get(nx_usize, ny_usize).unwrap_or(0.0);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::GridMap;

    #[test]
    fn test_in_bounds() {
        assert!(in_bounds(0, 0, 10));
        assert!(in_bounds(5, 5, 10));
        assert!(in_bounds(9, 9, 10));
        assert!(!in_bounds(-1, 0, 10));
        assert!(!in_bounds(0, -1, 10));
        assert!(!in_bounds(10, 0, 10));
        assert!(!in_bounds(0, 10, 10));
    }

    #[test]
    fn test_greedy_basic_path() {
        let mut grid = GridMap::from_file("grids/test_3x3.txt", 0.0).unwrap();
        let start = Position { x: 0, y: 0 };
        let path = greedy_lookahead(&mut grid, 5, 10000, start, 2);

        assert!(!path.is_empty());
        assert_eq!(path[0].x, 0);
        assert_eq!(path[0].y, 0);
    }

    #[test]
    fn test_greedy_respects_step_limit() {
        let mut grid = GridMap::from_file("grids/test_3x3.txt", 0.0).unwrap();
        let start = Position { x: 1, y: 1 };
        let path = greedy_lookahead(&mut grid, 3, 10000, start, 2);

        // Path length is start + steps taken (up to discrete_steps_count)
        assert!(path.len() <= 4); // start + 3 steps max
    }

    #[test]
    fn test_greedy_moves_to_high_value() {
        let mut grid = GridMap::from_file("grids/test_3x3.txt", 0.0).unwrap();
        // Start at (0,0) which has value 1.0
        // Adjacent cells: (0,1)=2.0, (1,0)=4.0, (1,1)=5.0
        // Should move toward higher values
        let start = Position { x: 0, y: 0 };
        let path = greedy_lookahead(&mut grid, 1, 10000, start, 2);

        assert_eq!(path.len(), 2);
        // With lookahead, it should pick the move that leads to best future value
        // (1,1) has value 5.0 and is adjacent to 9.0
        assert_eq!(path[1].x, 1);
        assert_eq!(path[1].y, 1);
    }
}
