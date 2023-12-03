use rand::{seq::SliceRandom, thread_rng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{network::Network, puzzle::Puzzle, utils::ArgMax};

#[derive(Serialize, Deserialize)]
pub enum ReplayStrategy {
    /// Attempts to give an even distribution of states close to being solved, and states further
    /// away from being solved. This is achieved by having `n` instances, each running parallel.
    /// Each instance takes a solved puzzle, feeds it into the network to create a replay
    /// observation, and then applies a random move to that cube (regardless of what the replay is)
    EvenSample {
        /// Maximum depth to scramble to
        scramble_depth: usize,
        /// Number of parallel instances to run
        instances: usize,
    },
    /// Takes a number of instances of randomly scrambled cubes, and gathers some number of replay
    /// observations from each instance. If it is solved, then it will scramble the cube again and
    /// continue building replay
    ScrambledState {
        /// Number of moves used to scramble the cube
        scramble_depth: usize,
        /// How many parallel instances
        instances: usize,
        /// How many observations should each instance gather
        instance_replay_length: usize,
    },
    RandomScrambleState {
        scramble_min: usize,
        scramble_max: usize,
        instances: usize,
        instance_replay_length: usize,
    },
}

impl ReplayStrategy {
    pub fn build_replay<P>(&self, network: &Network<P>, epsilon: f64) -> Vec<ReplayObservation<P>>
    where
        P: Puzzle + Send + Sync,
    {
        match self {
            ReplayStrategy::EvenSample {
                scramble_depth,
                instances,
            } => (0..*instances)
                .into_par_iter()
                .map(|_| {
                    let mut puzzle = P::new();
                    let mut rng = thread_rng();

                    (0..*scramble_depth)
                        .map(|_| {
                            puzzle
                                .apply(*puzzle.get_valid_actions().choose(&mut rng).unwrap())
                                .unwrap();

                            let mut state = puzzle.clone();

                            let action = if rng.gen_bool(epsilon) {
                                rng.gen_range(0..P::ACTIONS_LENGTH)
                            } else {
                                network.apply(puzzle.clone()).arg_max()
                            };

                            state.apply(action).unwrap();

                            ReplayObservation {
                                state: puzzle.clone(),
                                action,
                                reward: state.get_reward(),
                                next_state: state,
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>(),
            Self::ScrambledState {
                scramble_depth,
                instances,
                instance_replay_length,
            } => (0..*instances)
                .into_par_iter()
                .map(|_| {
                    let mut puzzle = P::new();
                    let mut rng = thread_rng();

                    (0..*instance_replay_length)
                        .map(|_| {
                            if puzzle.is_solved() {
                                for _ in 0..*scramble_depth {
                                    puzzle
                                        .apply(
                                            *puzzle.get_valid_actions().choose(&mut rng).unwrap(),
                                        )
                                        .unwrap();
                                }
                            }

                            let state = puzzle.clone();

                            let action = if rng.gen_bool(epsilon) {
                                rng.gen_range(0..P::ACTIONS_LENGTH)
                            } else {
                                network.apply(puzzle.clone()).arg_max()
                            };

                            puzzle.apply(action).unwrap();

                            ReplayObservation {
                                state,
                                action,
                                reward: puzzle.get_reward(),
                                next_state: puzzle.clone(),
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>(),
            Self::RandomScrambleState {
                scramble_min,
                scramble_max,
                instances,
                instance_replay_length,
            } => (0..*instances)
                .into_par_iter()
                .map(|_| {
                    let mut puzzle = P::new();
                    let mut rng = thread_rng();

                    (0..*instance_replay_length)
                        .map(|_| {
                            if puzzle.is_solved() {
                                for _ in 0..rng.gen_range(*scramble_min..=*scramble_max) {
                                    puzzle
                                        .apply(
                                            *puzzle.get_valid_actions().choose(&mut rng).unwrap(),
                                        )
                                        .unwrap();
                                }
                            }

                            let state = puzzle.clone();

                            let action = if rng.gen_bool(epsilon) {
                                rng.gen_range(0..P::ACTIONS_LENGTH)
                            } else {
                                network.apply(puzzle.clone()).arg_max()
                            };

                            puzzle.apply(action).unwrap();

                            ReplayObservation {
                                state,
                                action,
                                reward: puzzle.get_reward(),
                                next_state: puzzle.clone(),
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect(),
        }
    }

    /// Returns the minimum guarunteed observations
    pub fn get_min_observations(&self) -> usize {
        match self {
            Self::EvenSample {
                scramble_depth,
                instances,
            } => scramble_depth * instances,
            Self::ScrambledState {
                scramble_depth: _,
                instances,
                instance_replay_length,
            } => instances * instance_replay_length,
            Self::RandomScrambleState {
                scramble_min: _,
                scramble_max: _,
                instances,
                instance_replay_length,
            } => instances * instance_replay_length,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ReplayObservation<P>
where
    P: Puzzle,
{
    pub state: P,
    pub action: usize,
    pub reward: f64,
    pub next_state: P,
}
