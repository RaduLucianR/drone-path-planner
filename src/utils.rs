use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Deserialize)]
pub struct PathPlanningParams {
    pub maximum_duration_millis: u32,
    pub discrete_steps_count: usize,
    pub starting_position: Position,
}

#[derive(Debug, Deserialize)]
pub struct MapParams {
    pub map_file_path: String,
    pub replenish_rate: f32,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "path_planning_params")]
    pub path_planning_params: PathPlanningParams,
    #[serde(rename = "map_params")]
    pub map_params: MapParams,
}
