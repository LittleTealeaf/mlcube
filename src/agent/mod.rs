mod function;
mod replay;

pub use function::*;
use rand::{distributions::uniform::SampleRange, seq::IteratorRandom, thread_rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
pub use replay::*;

use serde::{Deserialize, Serialize};

use crate::{
    network::{Network, SolveResult},
    puzzle::Puzzle,
    utils::Max,
};

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
    epsilon: FnValue,
    alpha: FnValue,
}

impl<P> Agent<P>
where
    P: Puzzle + Sync + Send,
{
    pub fn new<R>(config: NewAgentConfig<R>) -> Result<Self, AgentConfigError>
    where
        R: SampleRange<f64> + Clone,
    {
        if config.replay_strategy.get_min_observations() < config.train_size {
            return Err(AgentConfigError::NotEnoughReplay {
                train_size: config.train_size,
                min_replay_size: config.replay_strategy.get_min_observations(),
            });
        }

        let mut network = Network::new(config.hidden_layers);
        network.randomize(&mut thread_rng(), config.initialize_range);

        Ok(Self {
            target: network.clone(),
            network,
            epoch: 0,
            gamma: config.gamma,
            train_size: config.train_size,
            update_interval: config.update_interval,
            replay_strategy: config.replay_strategy,
            epsilon: config.epsilon,
            alpha: config.alpha,
        })
    }

    pub fn train_epoch(&mut self) {
        let variables = FunctionVariables {
            epoch: self.epoch,
            update_interval: self.update_interval,
        };
        let alpha = self.alpha.calculate(&variables);
        let epsilon = self.epsilon.calculate(&variables);
        let mut rng = thread_rng();
        let replay = self.replay_strategy.build_replay(&self.network, epsilon);

        let nudges = replay
            .into_iter()
            .choose_multiple(&mut rng, self.train_size)
            .into_par_iter()
            .map(|observation| {
                let expected = observation.reward
                    + self.gamma * self.target.apply(observation.next_state).max();
                self.network.back_propagate(
                    observation.state,
                    observation.action,
                    expected,
                    alpha / (self.train_size as f64),
                )
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

    pub fn has_inf_or_nan(&self) -> bool {
        self.network.has_inf_or_nan()
    }

    pub fn solve(&self, puzzle: P, max_moves: usize) -> SolveResult {
        self.network.solve(puzzle, max_moves)
    }

    pub fn get_network(&self) -> &Network<P> {
        &self.network
    }
}

pub struct NewAgentConfig<R>
where
    R: SampleRange<f64> + Clone,
{
    pub hidden_layers: Vec<usize>,
    pub gamma: f64,
    pub update_interval: usize,
    pub replay_strategy: ReplayStrategy,
    pub train_size: usize,
    pub epsilon: FnValue,
    pub alpha: FnValue,
    pub initialize_range: R,
}

#[derive(Debug)]
pub enum AgentConfigError {
    /// There isn't enough replay to provide the required train size
    NotEnoughReplay {
        min_replay_size: usize,
        train_size: usize,
    },
}
