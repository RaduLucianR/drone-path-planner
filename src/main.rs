mod algorithms;
mod map;
mod utils;
mod visualize;

use std::error::Error;
use std::fs;

use algorithms::greedy_lookahead::plan_path;
use map::GridMap;
use utils::Config;
use visualize::draw_grid_with_path;

fn main() -> Result<(), Box<dyn Error>> {
    let default_config_path = "./config/config.toml";
    let config_str = fs::read_to_string(default_config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    let mut grid_map = GridMap::from_file(
        &config.map_params.map_file_path,
        config.map_params.replenish_rate,
    )?;

    let path = plan_path(
        &mut grid_map,
        config.path_planning_params.discrete_steps_count,
        config.path_planning_params.maximum_duration_millis,
        config.path_planning_params.starting_position,
        4,
    );

    let _ = draw_grid_with_path(&grid_map, &path, "output.png");
    println!("Score: {}", grid_map.get_path_score(path));

    Ok(())
}
