use chrono;
use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;

use drone_path_planner::{Config, GridMap, PathPlanningAlgorithm, draw_grid_with_path};

#[derive(Parser)]
#[command(name = "drone-path-planner")]
#[command(about = "Path planning algorithm", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "./config/config.toml")]
    config: String,

    /// Disable PNG visualization output
    #[arg(long)]
    no_png: bool,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(&cli.config, cli.no_png) {
        log::error!("{}", e);
        process::exit(1);
    }
}

fn run(config_path: &str, no_png: bool) -> Result<(), String> {
    setup_logging().map_err(|e| format!("Failed to setup logging: {}", e))?;
    log::info!("Starting process");

    let default_output_folder_path = "./output/";

    let config_str = fs::read_to_string(config_path).map_err(|e| {
        format!(
            "Config file not found at '{}'. Use --config to specify a different path.\nDetails: {}",
            config_path, e
        )
    })?;

    let config: Config = toml::from_str(&config_str)
        .map_err(|e| format!("Failed to parse config file '{}': {}", config_path, e))?;

    let mut grid_map = GridMap::from_file(
        &config.map_params.map_file_path,
        config.map_params.replenish_rate,
    )
    .map_err(|e| {
        format!(
            "Failed to load grid file '{}': {}",
            config.map_params.map_file_path, e
        )
    })?;

    let start = &config.path_planning_params.starting_position;
    let grid_size = grid_map.get_size();
    if start.x >= grid_size || start.y >= grid_size {
        return Err(format!(
            "Starting position ({}, {}) is out of bounds for grid of size {}x{}",
            start.x, start.y, grid_size, grid_size
        ));
    }

    let algorithm = PathPlanningAlgorithm::from_config(&config.path_planning_params)?;

    let result = algorithm.plan_path(
        &mut grid_map,
        config.path_planning_params.discrete_steps_count,
        config.path_planning_params.maximum_duration_millis,
        config.path_planning_params.starting_position,
    );

    if result.path.is_empty() {
        log::warn!("No path was generated");
        return Ok(());
    }

    let now = chrono::offset::Local::now();
    let now_str = now.format("%Y%m%d_%H%M%S");
    let txt_output_file_name = format!("{}{}.txt", default_output_folder_path, now_str);

    if !no_png {
        let png_output_file_name = format!("{}{}.png", default_output_folder_path, now_str);
        draw_grid_with_path(&grid_map, &result.path, &png_output_file_name)
            .map_err(|e| format!("Failed to write PNG output '{}': {}", png_output_file_name, e))?;
    }

    log::info!("Score: {}", result.score);

    let mut txt_file = File::create(&txt_output_file_name)
        .map_err(|e| format!("Failed to create output file '{}': {}", txt_output_file_name, e))?;

    let mut output_str = format!("{} {}\n", result.score, result.path.len());
    for pos in &result.path {
        output_str += &format!("{} {}\n", pos.x, pos.y);
    }

    txt_file
        .write(output_str.as_bytes())
        .map_err(|e| format!("Failed to write to '{}': {}", txt_output_file_name, e))?;

    Ok(())
}

fn setup_logging() -> Result<(), fern::InitError> {
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
    Ok(())
}
