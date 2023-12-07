use serde::{Deserialize, Serialize};

use crate::puzzle::Puzzle;

use super::Agent;

#[derive(Serialize, Deserialize)]
pub enum UpdateStrategy {
    Interval(usize),
    Threshold {
        initial_update_epoch: usize,
        test_size: usize,
        minimum_update_interval: usize,
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
            UpdateStrategy::Threshold {
                initial_update_epoch,
                test_size,
                threshold,
                minimum_update_interval,
            } => {
                if agent.epoch <= *initial_update_epoch {
                    agent.epoch == *initial_update_epoch
                } else {
                    agent.epoch - agent.last_target_update >= *minimum_update_interval && {
                        let error = agent.test_target_error(*test_size);
                        error < *threshold
                    }
                }
            }
        }
    }
}
