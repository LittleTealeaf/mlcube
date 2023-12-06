use serde::{Serialize, Deserialize};

use crate::puzzle::Puzzle;

use super::Agent;

#[derive(Serialize, Deserialize)]
pub enum UpdateStrategy {
    Interval(usize),
    Threshold {
        initial_update_epoch: usize,
        test_size: usize,
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
            } => {
                if agent.epoch <= *initial_update_epoch {
                    agent.epoch == *initial_update_epoch
                } else {
                    let error = agent.test_target_error(*test_size);
                    error < *threshold
                }
            }
        }
    }
}
