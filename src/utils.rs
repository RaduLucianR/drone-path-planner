use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Position {
    x: u64,
    y: u64,
}

#[derive(Debug, Deserialize)]
pub struct PathPlanningParams {
    maximum_duration_millis: u64,
    discrete_steps_count: usize,
    starting_position: Position,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "path_planning_params")]
    pub path_planning_params: PathPlanningParams,
}
