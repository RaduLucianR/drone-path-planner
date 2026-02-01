use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    original: i32,
    current: i32,
}

#[derive(Debug)]
pub struct GridMap {
    size: usize,
    replenish_rate: f32,
    grid: Vec<Vec<Cell>>,
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
                    string.parse::<i32>().map(|value| Cell {
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
        })
    }

    pub fn print(&self, original: bool) {
        for row in &self.grid {
            for cell in row {
                if original {
                    print!("{} ", cell.original);
                } else {
                    print!("{} ", cell.current);
                }
            }
            println!();
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get(&self, x: usize, y: usize) -> Option<i32> {
        self.grid
            .get(x)
            .and_then(|row| row.get(y))
            .map(|cell| cell.current)
    }

    pub fn visit(&mut self, x: usize, y: usize) {
        if let Some(row) = self.grid.get_mut(x) {
            if let Some(cell) = row.get_mut(y) {
                cell.current = 0;
            }
        }
    }

    pub fn replenish(&mut self) {
        for row in &mut self.grid {
            for cell in row {
                cell.current = ((cell.current as f32 + self.replenish_rate * cell.original as f32)
                    .round() as i32)
                    .min(cell.original);
            }
        }
    }
}
