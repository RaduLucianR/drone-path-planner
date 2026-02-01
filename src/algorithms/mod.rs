pub mod greedy_lookahead;
pub mod limited_beam_search;

use crate::algorithms::{
    greedy_lookahead::greedy_lookahead, limited_beam_search::limited_beam_search,
};
use crate::map::GridMap;
use crate::utils::{PathPlanningParams, Position};

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
