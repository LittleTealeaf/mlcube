use serde::{Deserialize, Serialize};

use crate::puzzle::Puzzle;

use super::Agent;

#[derive(Serialize, Deserialize)]
pub enum UpdateStrategy {
    Interval(usize),
    TrainThreshold {
        test_size: usize,
        initial_update: Option<usize>,
        min_update: Option<usize>,
        max_update: Option<usize>,
        threshold: f64,
    },
}

impl UpdateStrategy {
    pub fn do_update<P>(&self, agent: &Agent<P>) -> bool
    where
        P: Puzzle + Sync + Send,
    {
        match self {
            UpdateStrategy::Interval(interval) => agent.epoch % interval == 0,
            UpdateStrategy::TrainThreshold {
                test_size,
                initial_update,
                min_update,
                max_update,
                threshold,
            } => {
                if let Some(initial_update) = initial_update {
                    if agent.epoch <= *initial_update {
                        return agent.epoch == *initial_update;
                    }
                }

                if let Some(min_update) = min_update {
                    if agent.epoch - agent.last_target_update <= *min_update {
                        return false;
                    }
                }

                if let Some(max_update) = max_update {
                    if agent.epoch - agent.last_target_update >= *max_update {
                        return true;
                    }
                }

                let error = agent.test_target_error(*test_size);
                error < *threshold
            }
        }
    }
}
