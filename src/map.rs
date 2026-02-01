use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct GridMap {
    pub size: usize,
    pub grid: Vec<Vec<i32>>,
}

impl GridMap {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let last_row_length: Option<usize> = None;

        let mut grid: Vec<Vec<i32>> = Vec::new();

        for (line_number, line) in reader.lines().enumerate() {
            let line = line?;

            if line.trim().is_empty() {
                continue;
            }

            let row: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>())
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
        Ok(GridMap { size, grid })
    }

    pub fn print(&self) {
        for row in &self.grid {
            for elem in row {
                print!("{} ", elem);
            }
            println!();
        }
    }
}
