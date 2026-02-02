use crate::map::GridMap;
use crate::utils::Position;

use plotters::prelude::*;

const GRAYS: [u8; 10] = [240, 220, 200, 180, 160, 140, 120, 100, 80, 60];

fn stepped_gray(v: f32) -> RGBColor {
    let v = v.clamp(0.0, 1.0);
    let idx = (v * (GRAYS.len() as f32 - 1.0)).round() as usize;
    let g = GRAYS[idx];
    RGBColor(g, g, g)
}

pub fn draw_grid_with_path(
    grid: &GridMap,
    path: &Vec<Position>,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if path.len() == 0 {
        log::warn!("The path given for visualization has length zero! Exiting early.");
        return Ok(());
    }

    let cell_size: u32 = 20;
    let n = grid.get_size();

    let root = BitMapBackend::new(
        filename,
        (n as u32 * cell_size, n as u32 * cell_size), // 20 px per cell
    )
    .into_drawing_area();

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
        .build_cartesian_2d(0f32..(n as f32), (n as f32)..0f32)?;

    chart.configure_mesh().disable_mesh().draw()?;

    // Heatmap
    for x in 0..n {
        for y in 0..n {
            let value = grid.get_original(x, y).unwrap_or(0.0) as f32 / max_val as f32;

            let color = stepped_gray(value);

            chart.draw_series(std::iter::once(Rectangle::new(
                [(y as f32, x as f32), ((y as f32 + 1.0), (x as f32 + 1.0))],
                color.filled(),
            )))?;
        }
    }

    // Path overlay through cell centers
    let path_points: Vec<(f32, f32)> = path
        .iter()
        .map(|p| ((p.y as f32 + 0.5), (p.x as f32 + 0.5)))
        .collect();

    chart.draw_series(LineSeries::new(
        path_points.clone(),
        ShapeStyle::from(&RED).stroke_width(1),
    ))?;

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
