pub mod greedy_lookahead;
pub mod limited_beam_search;

use crate::algorithms::{
    greedy_lookahead::greedy_lookahead, limited_beam_search::limited_beam_search,
};
use crate::map::GridMap;
use crate::utils::{PathPlanningParams, Position};

#[derive(Debug)]
pub enum PathPlanningAlgorithm {
    GreedyLookahead { lookahead: usize },
    LimitedBeamSearch { lookahead: usize, beam_width: usize },
}

impl PathPlanningAlgorithm {
    pub fn from_config(params: &PathPlanningParams) -> Result<PathPlanningAlgorithm, String> {
        let algo_params = params
            .algo_specific_params
            .ok_or("algo_specific_params is required")?;

        match params.algorithm.as_str() {
            "greedy_lookahead" => Ok(PathPlanningAlgorithm::GreedyLookahead {
                lookahead: algo_params.require_lookahead()?,
            }),
            "limited_beam_search" => Ok(PathPlanningAlgorithm::LimitedBeamSearch {
                lookahead: algo_params.require_lookahead()?,
                beam_width: algo_params.require_beam_width()?,
            }),
            other => Err(format!("Unknown algorithm: {}", other)),
        }
    }

    pub fn plan_path(
        &self,
        grid_map: &mut GridMap,
        discrete_steps_count: u32,
        maximum_duration_millis: u64,
        start: Position,
    ) -> Vec<Position> {
        match self {
            Self::GreedyLookahead { lookahead } => greedy_lookahead(
                grid_map,
                discrete_steps_count,
                maximum_duration_millis,
                start,
                *lookahead,
            ),
            Self::LimitedBeamSearch {
                lookahead,
                beam_width,
            } => limited_beam_search(
                grid_map,
                discrete_steps_count,
                maximum_duration_millis,
                start,
                *lookahead,
                *beam_width,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::AlgoSpecificParams;

    fn make_params(algorithm: &str, lookahead: Option<usize>, beam_width: Option<usize>) -> PathPlanningParams {
        PathPlanningParams {
            maximum_duration_millis: 1000,
            discrete_steps_count: 10,
            starting_position: Position { x: 0, y: 0 },
            algorithm: algorithm.to_string(),
            algo_specific_params: Some(AlgoSpecificParams { lookahead, beam_width }),
        }
    }

    #[test]
    fn test_from_config_greedy_lookahead() {
        let params = make_params("greedy_lookahead", Some(5), None);
        let algo = PathPlanningAlgorithm::from_config(&params).unwrap();
        match algo {
            PathPlanningAlgorithm::GreedyLookahead { lookahead } => {
                assert_eq!(lookahead, 5);
            }
            _ => panic!("Expected GreedyLookahead"),
        }
    }

    #[test]
    fn test_from_config_limited_beam_search() {
        let params = make_params("limited_beam_search", Some(8), Some(3));
        let algo = PathPlanningAlgorithm::from_config(&params).unwrap();
        match algo {
            PathPlanningAlgorithm::LimitedBeamSearch { lookahead, beam_width } => {
                assert_eq!(lookahead, 8);
                assert_eq!(beam_width, 3);
            }
            _ => panic!("Expected LimitedBeamSearch"),
        }
    }

    #[test]
    fn test_from_config_unknown_algorithm() {
        let params = make_params("unknown_algo", Some(5), Some(3));
        let result = PathPlanningAlgorithm::from_config(&params);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown algorithm"));
    }

    #[test]
    fn test_from_config_missing_params() {
        let params = PathPlanningParams {
            maximum_duration_millis: 1000,
            discrete_steps_count: 10,
            starting_position: Position { x: 0, y: 0 },
            algorithm: "greedy_lookahead".to_string(),
            algo_specific_params: None,
        };
        let result = PathPlanningAlgorithm::from_config(&params);
        assert!(result.is_err());
    }
}
