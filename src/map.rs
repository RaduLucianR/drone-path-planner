use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::utils::Position;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    original: f32,
    current: f32,
}

#[derive(Debug)]
pub struct GridMap {
    size: usize,
    replenish_rate: f32,
    grid: Vec<Vec<Cell>>,
    visited_cells: HashSet<(usize, usize)>,
}

impl GridMap {
    pub fn from_file<P: AsRef<Path>>(
        path: P,
        replenish_rate: f32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let last_row_length: Option<usize> = None;

        let mut grid: Vec<Vec<Cell>> = Vec::new();

        for (line_number, line) in reader.lines().enumerate() {
            let line = line?;

            if line.trim().is_empty() {
                continue;
            }

            let row: Vec<Cell> = line
                .split_whitespace()
                .map(|string| {
                    string.parse::<f32>().map(|value| Cell {
                        original: value,
                        current: value,
                    })
                })
                .collect::<Result<_, _>>()?;

            if let Some(len) = last_row_length {
                if row.len() != len {
                    return Err(format!(
                        "The input grid is not a square! Row {} has length {}, expected {}",
                        line_number,
                        row.len(),
                        len
                    )
                    .into());
                }
            }

            grid.push(row);
        }

        let size = grid.len();
        Ok(GridMap {
            size,
            grid,
            replenish_rate,
            visited_cells: HashSet::new(),
        })
    }

    pub fn print(&self, original: bool) {
        for row in &self.grid {
            let line: String = row
                .iter()
                .map(|cell| {
                    if original {
                        format!("{} ", cell.original)
                    } else {
                        format!("{} ", cell.current)
                    }
                })
                .collect();
            log::debug!("{}", line.trim_end());
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get(&self, x: usize, y: usize) -> Option<f32> {
        self.grid
            .get(x)
            .and_then(|row| row.get(y))
            .map(|cell| cell.current)
    }

    pub fn get_original(&self, x: usize, y: usize) -> Option<f32> {
        self.grid
            .get(x)
            .and_then(|row| row.get(y))
            .map(|c| c.original)
    }

    pub fn get_path_score(&self, path: &Vec<Position>) -> f32 {
        let mut score = 0.0;
        for pos in path {
            if let Some(val) = self
                .grid
                .get(pos.x)
                .and_then(|row| row.get(pos.y))
                .map(|cell| cell.original)
            {
                score += val;
            }
        }
        score
    }

    pub fn visit(&mut self, x: usize, y: usize) {
        if let Some(row) = self.grid.get_mut(x) {
            if let Some(cell) = row.get_mut(y) {
                cell.current = 0.0;
                self.visited_cells.insert((x, y));
            }
        }
    }

    pub fn replenish(&mut self) {
        let mut fully_replenished = Vec::new();

        for &(x, y) in &self.visited_cells {
            let cell = &mut self.grid[x][y];
            cell.current = (cell.current + self.replenish_rate * cell.original).min(cell.original);

            if (cell.current - cell.original).abs() < 0.001 {
                fully_replenished.push((x, y));
            }
        }

        for pos in fully_replenished {
            self.visited_cells.remove(&pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_file_valid() {
        let grid = GridMap::from_file("grids/test_3x3.txt", 0.1).unwrap();
        assert_eq!(grid.get_size(), 3);
        assert_eq!(grid.get(0, 0), Some(1.0));
        assert_eq!(grid.get(1, 1), Some(5.0));
        assert_eq!(grid.get(2, 2), Some(9.0));
    }

    #[test]
    fn test_get_and_visit() {
        let mut grid = GridMap::from_file("grids/test_3x3.txt", 0.1).unwrap();

        assert_eq!(grid.get(1, 1), Some(5.0));
        grid.visit(1, 1);
        assert_eq!(grid.get(1, 1), Some(0.0));
        assert_eq!(grid.get_original(1, 1), Some(5.0));
    }

    #[test]
    fn test_replenish() {
        let mut grid = GridMap::from_file("grids/test_3x3.txt", 0.5).unwrap();

        grid.visit(1, 1);
        assert_eq!(grid.get(1, 1), Some(0.0));

        grid.replenish();
        assert_eq!(grid.get(1, 1), Some(2.5)); // 0.5 * 5.0 = 2.5

        grid.replenish();
        assert_eq!(grid.get(1, 1), Some(5.0)); // 2.5 + 2.5 = 5.0, capped at original
    }

    #[test]
    fn test_get_path_score() {
        let grid = GridMap::from_file("grids/test_3x3.txt", 0.1).unwrap();
        let path = vec![
            Position { x: 0, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 2 },
        ];
        assert_eq!(grid.get_path_score(&path), 15.0); // 1 + 5 + 9
    }

    #[test]
    fn test_out_of_bounds() {
        let grid = GridMap::from_file("grids/test_3x3.txt", 0.1).unwrap();
        assert_eq!(grid.get(10, 10), None);
        assert_eq!(grid.get_original(10, 10), None);
    }
}
