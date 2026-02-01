pub mod greedy_lookahead;
pub mod limited_beam_search;

use crate::algorithms::{
    greedy_lookahead::greedy_lookahead, limited_beam_search::limited_beam_search,
};
use crate::map::GridMap;
use crate::utils::Position;

pub enum PathPlanningAlgorithm {
    GreedyLookahead { lookahead: usize },
    LimitedBeamSearch { lookahead: usize, beam_width: u32 },
}

impl PathPlanningAlgorithm {
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
            } => limited_beam_search(),
        }
    }
}
