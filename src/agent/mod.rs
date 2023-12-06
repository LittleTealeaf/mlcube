mod function;
mod replay_buffer;
mod sample_strategy;

pub use function::*;
use rand::{distributions::uniform::SampleRange, thread_rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
pub use replay_buffer::*;
pub use sample_strategy::*;

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
    sample_strategy: SampleStrategy,
    batch_size: usize,
    epsilon: FnValue,
    alpha: FnValue,
    replay: ReplayBuffer<P>,
    penalize_repeats: bool,
}

impl<P> Agent<P>
where
    P: Puzzle + Sync + Send,
{
    pub fn new<R>(config: NewAgentConfig<R>) -> Result<Self, AgentConfigError>
    where
        R: SampleRange<f64> + Clone,
    {
        if config.sample_strategy.get_min_observations() < config.batch_size {
            return Err(AgentConfigError::NotEnoughReplay {
                batch_size: config.batch_size,
                min_replay_size: config.sample_strategy.get_min_observations(),
            });
        }

        let mut network = Network::new(config.hidden_layers);
        network.randomize(&mut thread_rng(), config.initialize_range);

        Ok(Self {
            target: network.clone(),
            network,
            epoch: 0,
            gamma: config.gamma,
            batch_size: config.batch_size,
            update_interval: config.update_interval,
            sample_strategy: config.sample_strategy,
            epsilon: config.epsilon,
            alpha: config.alpha,
            replay: ReplayBuffer::new(config.max_replay_size),
            penalize_repeats: config.penalize_repeats,
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
        let sample_params = SampleParams {
            epsilon,
            epoch: self.epoch,
            update_interval: self.update_interval,
        };
        let replay = self
            .sample_strategy
            .build_replay(&self.network, sample_params);
        for observation in replay {
            self.replay.insert_observation(observation, &mut rng);
        }

        let nudges = self
            .replay
            .sample(self.batch_size, &mut rng)
            .into_par_iter()
            .map(|observation| {
                let expected = observation.reward
                    + if self.penalize_repeats && observation.state == observation.next_state {
                        0f64
                    } else {
                        self.gamma * self.target.apply(observation.next_state).max()
                    };
                self.network.back_propagate(
                    observation.state,
                    observation.action,
                    expected,
                    alpha / (self.batch_size as f64),
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

    pub fn test_target_error(&self, observations: usize) -> f64 {
        let mut rng = thread_rng();

        let sample = self.replay.sample(observations, &mut rng);

        sample
            .into_par_iter()
            .map(|replay| {
                let q_network = self.network.apply(replay.state)[replay.action];
                let reward = replay.reward;
                let target = self.target.apply(replay.next_state).max();
                let expected = reward + target * self.gamma;
                let error = q_network - expected;
                error.abs()
            })
            .sum::<f64>()
            / observations as f64
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
    pub sample_strategy: SampleStrategy,
    pub batch_size: usize,
    pub epsilon: FnValue,
    pub alpha: FnValue,
    pub initialize_range: R,
    pub max_replay_size: usize,
    pub penalize_repeats: bool,
}

#[derive(Debug)]
pub enum AgentConfigError {
    /// There isn't enough replay to provide the required train size
    NotEnoughReplay {
        min_replay_size: usize,
        batch_size: usize,
    },
}
