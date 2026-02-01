use crate::map::GridMap;
use crate::utils::Position;

use plotters::prelude::*;

pub fn draw_grid_with_path(
    grid: &GridMap,
    path: &Vec<Position>,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if path.len() == 0 {
        println!("The path given for visualization has length zero! Exiting early.");
        return Ok(());
    }

    let n = grid.get_size();

    let root = BitMapBackend::new(filename, (600, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Normalize colors using original values
    let mut max_val: f32 = 1.0;
    for x in 0..n {
        for y in 0..n {
            if let Some(v) = grid.get_original(x, y) {
                max_val = max_val.max(v);
            }
        }
    }

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .build_cartesian_2d(0f32..n as f32, (n as f32)..0f32)?;

    chart.configure_mesh().disable_mesh().draw()?;

    // Heatmap
    for x in 0..n {
        for y in 0..n {
            let value = grid.get_original(x, y).unwrap_or(0.0) as f32 / max_val as f32;

            let color = RGBColor(255, (200.0 * (1.0 - value)) as u8, 0);

            chart.draw_series(std::iter::once(Rectangle::new(
                [(y as f32, x as f32), (y as f32 + 1.0, x as f32 + 1.0)],
                color.filled(),
            )))?;
        }
    }

    // Path overlay through cell centers
    let path_points: Vec<(f32, f32)> = path
        .iter()
        .map(|p| (p.y as f32 + 0.5, p.x as f32 + 0.5))
        .collect();

    chart.draw_series(LineSeries::new(path_points.clone(), &CYAN))?;

    chart.draw_series(std::iter::once(Circle::new(
        path_points[0],
        4,
        BLUE.filled(),
    )))?;

    chart.draw_series(std::iter::once(Circle::new(
        *path_points.last().unwrap(),
        4,
        GREEN.filled(),
    )))?;

    root.present()?;
    Ok(())
}
