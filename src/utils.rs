use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct AlgoSpecificParams {
    pub lookahead: Option<usize>,
    pub beam_width: Option<usize>,
}

impl AlgoSpecificParams {
    pub fn require_lookahead(&self) -> Result<usize, String> {
        self.lookahead
            .ok_or("Parameter lookahead must be provided".to_string())
    }

    pub fn require_beam_width(&self) -> Result<usize, String> {
        self.beam_width
            .ok_or("Parameter beam_width must be provided".to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct PathPlanningParams {
    pub maximum_duration_millis: u64,
    pub discrete_steps_count: u32,
    pub starting_position: Position,
    pub algorithm: String,
    #[serde(rename = "algorithm_specific_parameters")]
    pub algo_specific_params: Option<AlgoSpecificParams>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_lookahead_present() {
        let params = AlgoSpecificParams {
            lookahead: Some(5),
            beam_width: None,
        };
        assert_eq!(params.require_lookahead().unwrap(), 5);
    }

    #[test]
    fn test_require_lookahead_missing() {
        let params = AlgoSpecificParams {
            lookahead: None,
            beam_width: None,
        };
        assert!(params.require_lookahead().is_err());
    }

    #[test]
    fn test_require_beam_width_present() {
        let params = AlgoSpecificParams {
            lookahead: None,
            beam_width: Some(3),
        };
        assert_eq!(params.require_beam_width().unwrap(), 3);
    }

    #[test]
    fn test_require_beam_width_missing() {
        let params = AlgoSpecificParams {
            lookahead: None,
            beam_width: None,
        };
        assert!(params.require_beam_width().is_err());
    }
}
