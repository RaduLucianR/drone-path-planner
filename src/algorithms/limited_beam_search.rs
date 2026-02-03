use crate::map::GridMap;
use crate::utils::{in_bounds, PathScore, Position};
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn beam_score_move(
    grid_map: &GridMap,
    x: usize,
    y: usize,
    depth: usize,
    beam_width: usize,
    directions: &[(isize, isize)],
    cache: &mut HashMap<(usize, usize, usize), f32>,
) -> f32 {
    if depth == 0 {
        return 0.0;
    }

    let key = (x, y, depth);
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    // Vec< score, x, y >
    let mut candidates: Vec<(f32, usize, usize)> = Vec::new();

    for &(dx, dy) in directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if !in_bounds(nx, ny, grid_map.get_size()) {
            continue;
        }

        let nx_usize = nx as usize;
        let ny_usize = ny as usize;
        let val = grid_map.get(nx_usize, ny_usize).unwrap_or(0.0);

        candidates.push((val, nx_usize, ny_usize));
    }

    // Sort the candidates by score in decreasing order
    candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    candidates.truncate(beam_width);

    let mut best: f32 = 0.0;
    for (val, nx, ny) in candidates {
        let future = beam_score_move(grid_map, nx, ny, depth - 1, beam_width, directions, cache);
        best = best.max(val + future);
    }

    cache.insert(key, best);
    best
}

pub fn limited_beam_search(
    grid_map: &mut GridMap,
    discrete_steps_count: u32,
    maximum_duration_millis: u64,
    start: Position,
    lookahead: usize,
    beam_width: usize,
) -> PathScore {
    let mut path = vec![start];
    let mut total_score = grid_map.get(start.x, start.y).unwrap_or(0.0);
    grid_map.visit(start.x, start.y);

    let mut x = start.x;
    let mut y = start.y;

    let directions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let start_time = Instant::now();
    let max_duration = Duration::from_millis(maximum_duration_millis);

    for _step in 0..discrete_steps_count {
        if start_time.elapsed() > max_duration {
            log::warn!("Time limit exceeded, exiting...");
            break;
        }

        grid_map.replenish();

        let mut cache: HashMap<(usize, usize, usize), f32> = HashMap::new();
        let mut best_move: Option<Position> = None;
        let mut best_value = f32::MIN;

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if !in_bounds(nx, ny, grid_map.get_size()) {
                continue;
            }

            let nx_usize = nx as usize;
            let ny_usize = ny as usize;

            let immediate = grid_map.get(nx_usize, ny_usize).unwrap_or(0.0);

            let future = beam_score_move(
                grid_map,
                nx_usize,
                ny_usize,
                lookahead - 1,
                beam_width,
                &directions,
                &mut cache,
            );

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
            let collected = grid_map.get(nx, ny).unwrap_or(0.0);
            total_score += collected;
            x = nx;
            y = ny;
            path.push(Position { x, y });
            grid_map.visit(x, y);
        } else {
            break;
        }
    }

    PathScore {
        path,
        score: total_score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::GridMap;

    #[test]
    fn test_beam_search_basic_path() {
        let mut grid = GridMap::from_file("grids/test_3x3.txt", 0.0).unwrap();
        let start = Position { x: 0, y: 0 };
        let result = limited_beam_search(&mut grid, 5, 10000, start, 2, 2);

        assert!(!result.path.is_empty());
        assert_eq!(result.path[0].x, 0);
        assert_eq!(result.path[0].y, 0);
        assert!(result.score > 0.0);
    }

    #[test]
    fn test_beam_search_respects_step_limit() {
        let mut grid = GridMap::from_file("grids/test_3x3.txt", 0.0).unwrap();
        let start = Position { x: 1, y: 1 };
        let result = limited_beam_search(&mut grid, 3, 10000, start, 2, 2);

        // Path length is start + steps taken (up to discrete_steps_count)
        assert!(result.path.len() <= 4); // start + 3 steps max
    }

    #[test]
    fn test_beam_search_moves_to_high_value() {
        let mut grid = GridMap::from_file("grids/test_3x3.txt", 0.0).unwrap();
        // Start at (0,0) which has value 1.0
        // Adjacent cells: (0,1)=2.0, (1,0)=4.0, (1,1)=5.0
        // Should move toward higher values
        let start = Position { x: 0, y: 0 };
        let result = limited_beam_search(&mut grid, 1, 10000, start, 2, 2);

        assert_eq!(result.path.len(), 2);
        // With lookahead, it should pick the move that leads to best future value
        assert_eq!(result.path[1].x, 1);
        assert_eq!(result.path[1].y, 1);
    }

    #[test]
    fn test_beam_width_affects_search() {
        // With beam_width=1, only the single best candidate is explored at each depth
        // With beam_width=8, all candidates are explored (like greedy_lookahead)
        let mut grid1 = GridMap::from_file("grids/test_3x3.txt", 0.0).unwrap();
        let mut grid2 = GridMap::from_file("grids/test_3x3.txt", 0.0).unwrap();

        let start = Position { x: 1, y: 1 };
        let result_narrow = limited_beam_search(&mut grid1, 2, 10000, start, 3, 1);
        let result_wide = limited_beam_search(&mut grid2, 2, 10000, start, 3, 8);

        // Both should produce valid paths
        assert!(!result_narrow.path.is_empty());
        assert!(!result_wide.path.is_empty());
    }
}
