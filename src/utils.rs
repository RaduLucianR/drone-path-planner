use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Deserialize)]
pub struct PathPlanningParams {
    pub maximum_duration_millis: u64,
    pub discrete_steps_count: u32,
    pub starting_position: Position,
}

#[derive(Debug, Deserialize)]
pub struct MapParams {
    pub map_file_path: String,
    pub replenish_rate: i32,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "path_planning_params")]
    pub path_planning_params: PathPlanningParams,
    #[serde(rename = "map_params")]
    pub map_params: MapParams,
}
