use rand::distributions::uniform::SampleRange;

use crate::puzzle::Puzzle;

use super::{Agent, AgentConfigError, ReplayStrategy, Value};

pub struct AgentFactory<R>
where
    R: SampleRange<f64> + Clone,
{
    pub hidden_layers: Vec<usize>,
    pub gamma: f64,
    pub update_interval: usize,
    pub replay_strategy: ReplayStrategy,
    pub train_size: usize,
    /// The chance that the algorithm will make a random move
    pub epsilon: Value,
    /// How fast the algorithm learns
    pub alpha: Value,
    /// Range of values to randomize network with
    pub initialize_range: R,
}

impl<R> AgentFactory<R>
where
    R: SampleRange<f64> + Clone,
{
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
            initialize_range,
        } = self;

        Agent::new(
            hidden_layers,
            gamma,
            update_interval,
            replay_strategy,
            train_size,
            epsilon,
            alpha,
            initialize_range,
        )
    }
}
