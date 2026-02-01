use crate::map::GridMap;
use crate::utils::Position;
use std::time::{Duration, Instant};

fn in_bounds(nx: isize, ny: isize, bound: isize) -> bool {
    nx >= 0 && ny >= 0 && nx < bound && ny < bound
}

// Beam search version of score_move
fn beam_score_move(
    grid_map: &GridMap,
    x: usize,
    y: usize,
    depth: usize,
    beam_width: usize,
    directions: &[(isize, isize)],
) -> f32 {
    if depth == 0 {
        return 0.0;
    }

    // Generate all candidates with their immediate values
    let mut candidates: Vec<(f32, usize, usize)> = Vec::new();

    for &(dx, dy) in directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if !in_bounds(nx, ny, grid_map.get_size() as isize) {
            continue;
        }

        let nx_usize = nx as usize;
        let ny_usize = ny as usize;
        let val = grid_map.get(nx_usize, ny_usize).unwrap_or(0.0);

        candidates.push((val, nx_usize, ny_usize));
    }

    // BEAM SEARCH: Sort by value and keep only top beam_width
    candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    candidates.truncate(beam_width);

    // Recursively explore only the best candidates
    let mut best: f32 = 0.0;
    for (val, nx, ny) in candidates {
        let future = beam_score_move(grid_map, nx, ny, depth - 1, beam_width, directions);
        best = best.max(val + future);
    }

    best
}

pub fn limited_beam_search(
    grid_map: &mut GridMap,
    discrete_steps_count: u32,
    maximum_duration_millis: u64,
    start: Position,
    lookahead: usize,
    beam_width: usize,
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
            println!("Time limit exceeded, exiting...");
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

            // Use beam search instead of full lookahead
            let future = beam_score_move(
                grid_map,
                nx_usize,
                ny_usize,
                lookahead - 1,
                beam_width,
                &directions,
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
