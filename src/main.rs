mod algorithms;
mod map;
mod utils;
mod visualize;

use chrono;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;

use algorithms::PathPlanningAlgorithm;
use map::GridMap;
use utils::Config;
use visualize::draw_grid_with_path;

fn main() -> Result<(), Box<dyn Error>> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    log::info!("Starting process");

    let default_config_path = "./config/config.toml";
    let default_output_folder_path = "./output/";

    let config_str = fs::read_to_string(default_config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    let mut grid_map = GridMap::from_file(
        &config.map_params.map_file_path,
        config.map_params.replenish_rate,
    )?;

    let algorithm = PathPlanningAlgorithm::from_config(&config.path_planning_params)?;

    let path = algorithm.plan_path(
        &mut grid_map,
        config.path_planning_params.discrete_steps_count,
        config.path_planning_params.maximum_duration_millis,
        config.path_planning_params.starting_position,
    );

    if path.len() == 0 {
        return Ok(());
    }

    let now = chrono::offset::Local::now();
    let now_str = now.format("%Y%m%d_%H%M%S");
    let png_output_file_name = format!("{}{}.png", default_output_folder_path, now_str);
    let txt_output_file_name = format!("{}{}.txt", default_output_folder_path, now_str);
    let _ = draw_grid_with_path(&grid_map, &path, &png_output_file_name);

    let score = grid_map.get_path_score(&path);
    log::info!("Score: {}", score);

    let mut txt_file = File::create(txt_output_file_name)?;
    let mut output_str = format!("{} {}\n", score, path.len());
    for pos in path {
        output_str += &format!("{} {}\n", pos.x, pos.y);
    }
    txt_file.write(output_str.as_bytes())?;

    Ok(())
}
