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
pub struct Files {
    pub map_file_path: String,
    pub output_folder_path: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "path_planning_params")]
    pub path_planning_params: PathPlanningParams,
    #[serde(rename = "files")]
    pub files: Files,
}
