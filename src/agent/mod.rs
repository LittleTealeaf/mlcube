mod epoch_function;
mod replay;

pub use epoch_function::*;
use rand::thread_rng;
pub use replay::*;

use serde::{Deserialize, Serialize};

use crate::{network::Network, puzzle::Puzzle};

#[derive(Serialize, Deserialize)]
pub struct Agent<P>
where
    P: Puzzle,
{
    network: Network<P>,
    target: Network<P>,
    epoch: usize,
    gamma: f64,
    update_interval: usize,
    replay_strategy: ReplayStrategy,
    train_size: usize,
    epsilon: EpochFunction,
    alpha: EpochFunction,
}

impl<P> Agent<P>
where
    P: Puzzle,
{
    pub fn new(
        hidden_layers: Vec<usize>,
        gamma: f64,
        update_interval: usize,
        replay_strategy: ReplayStrategy,
        train_size: usize,
        epsilon: EpochFunction,
        alpha: EpochFunction,
    ) -> Result<Self, AgentConfigError> {
        if replay_strategy.get_min_observations() < train_size {
            return Err(AgentConfigError::NotEnoughReplay {
                train_size,
                min_replay_size: replay_strategy.get_min_observations(),
            });
        }

        let mut network = Network::new(hidden_layers);
        network.randomize(&mut thread_rng(), -0.1..0.1);

        Ok(Self {
            target: network.clone(),
            network,
            epoch: 0,
            gamma,
            train_size,
            update_interval,
            replay_strategy,
            epsilon,
            alpha,
        })
    }
}

#[derive(Debug)]
pub enum AgentConfigError {
    /// There isn't enough replay to provide the required train size
    NotEnoughReplay {
        min_replay_size: usize,
        train_size: usize,
    },
}
