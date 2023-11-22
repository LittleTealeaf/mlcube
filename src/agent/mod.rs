mod epoch_function;
mod replay;
mod factory;

pub use replay::*;
pub use epoch_function::*;
pub use factory::*;
use rand::{seq::IteratorRandom, thread_rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use serde::{Deserialize, Serialize};

use crate::{network::Network, puzzle::Puzzle, utils::Max};

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
    P: Puzzle + Sync + Send,
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

    pub fn train_epoch(&mut self) {
        let alpha = self.alpha.calculate(self.epoch, self.update_interval);
        let epsilon = self.epsilon.calculate(self.epoch, self.update_interval);
        let mut rng = thread_rng();
        let replay = self
            .replay_strategy
            .build_replay(&self.network, epsilon / (self.train_size as f64));

        let nudges = replay
            .into_iter()
            .choose_multiple(&mut rng, self.train_size)
            .into_par_iter()
            .map(|observation| {
                let expected = observation.reward
                    + self.gamma * self.target.apply(observation.next_state).max();
                self.network
                    .back_propagate(observation.state, observation.action, expected, alpha)
            })
            .reduce(Vec::new, |mut a, b| {
                if a.is_empty() {
                    b
                } else if b.is_empty() {
                    a
                } else {
                    for (i, layer) in b.into_iter().enumerate() {
                        a[i].add(layer);
                    }
                    a
                }
            });

            self.network.update_weights(nudges);

            if self.epoch % self.update_interval == 0 {
                self.target = self.network.clone();
            }

            self.epoch += 1;
    }

    pub fn get_epoch(&self) -> usize {
        self.epoch
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
