use crate::puzzle::Puzzle;

use super::{Agent, AgentConfigError, EpochFunction, ReplayStrategy};

pub struct AgentFactory {
    pub hidden_layers: Vec<usize>,
    pub gamma: f64,
    pub update_interval: usize,
    pub replay_strategy: ReplayStrategy,
    pub train_size: usize,
    pub epsilon: EpochFunction,
    pub alpha: EpochFunction,
}

impl AgentFactory {
    pub fn build<P>(self) -> Result<Agent<P>, AgentConfigError>
    where
        P: Puzzle + Sync + Send,
    {
        let AgentFactory {
            hidden_layers,
            gamma,
            update_interval,
            replay_strategy,
            train_size,
            epsilon,
            alpha,
        } = self;

        Agent::new(
            hidden_layers,
            gamma,
            update_interval,
            replay_strategy,
            train_size,
            epsilon,
            alpha,
        )
    }
}
