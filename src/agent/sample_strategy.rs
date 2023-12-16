use rand::{seq::SliceRandom, thread_rng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{network::Network, puzzle::Puzzle, utils::ArgMax};

use super::ReplayObservation;

#[derive(Serialize, Deserialize)]
pub enum SampleStrategy {
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
    /// Scramble depth based on the number of target updates
    Iterative {
        target_updates_per_step: usize,
        instances: usize,
        instance_replay_length: usize,
    },
    /// Scramble depth based on number of target updates, only allows the agent to explore a number
    /// of steps equal to the scramble depth before it gets reset.
    ///
    /// This complication will hopefully make the dataset more saturated with near-solved states
    /// early on, forcing it to learn early and extend later
    ForcedIterative {
        target_updates_per_step: usize,
        instances: usize,
        instance_replay_length: usize,
    },
    /// Like Forced Iterative except now we look at every possible state as a new observation, so
    /// multiplying our observations by the move count
    ForcedIterativeBreadth {
        target_updates_per_step: usize,
        instances: usize,
        instance_replay_length: usize,
    },
}

impl SampleStrategy {
    pub fn build_replay<P>(
        &self,
        network: &Network<P>,
        SampleParams {
            epsilon,
            last_target_update: _,
            target_update_count,
            epoch: _,
        }: SampleParams,
    ) -> Vec<ReplayObservation<P>>
    where
        P: Puzzle + Send + Sync,
    {
        match self {
            Self::EvenSample {
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
            Self::Iterative {
                target_updates_per_step,
                instances,
                instance_replay_length,
            } => (0..*instances)
                .into_par_iter()
                .map(|_| {
                    let mut puzzle = P::new();
                    let mut rng = thread_rng();
                    let scramble = target_update_count / target_updates_per_step;

                    (0..*instance_replay_length)
                        .map(|_| {
                            if puzzle.is_solved() {
                                for _ in 0..rng.gen_range(0..=scramble) {
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
            Self::ForcedIterative {
                target_updates_per_step,
                instances,
                instance_replay_length,
            } => (0..*instances)
                .into_par_iter()
                .map(|_| {
                    let mut puzzle = P::new();
                    let mut rng = thread_rng();
                    let scramble = target_update_count / target_updates_per_step + 1;
                    let mut moves = 0;

                    (0..*instance_replay_length)
                        .map(|_| {
                            if puzzle.is_solved() || moves >= scramble {
                                puzzle = P::new();
                                for _ in 0..scramble {
                                    puzzle
                                        .apply(
                                            *puzzle.get_valid_actions().choose(&mut rng).unwrap(),
                                        )
                                        .unwrap();
                                }
                                moves = 0;
                            }
                            moves += 1;

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

            Self::ForcedIterativeBreadth {
                target_updates_per_step,
                instances,
                instance_replay_length,
            } => (0..*instances)
                .into_par_iter()
                .map(|_| {
                    let mut puzzle = P::new();
                    let mut rng = thread_rng();
                    let scramble = target_update_count / target_updates_per_step + 1;
                    let mut moves = 0;

                    (0..*instance_replay_length)
                        .flat_map(|_| {
                            if puzzle.is_solved() || moves >= scramble {
                                puzzle = P::new();
                                for _ in 0..scramble {
                                    puzzle
                                        .apply(
                                            *puzzle.get_valid_actions().choose(&mut rng).unwrap(),
                                        )
                                        .unwrap();
                                }
                                moves = 0;
                            }
                            moves += 1;

                            let state = puzzle.clone();

                            let action = if rng.gen_bool(epsilon) {
                                rng.gen_range(0..P::ACTIONS_LENGTH)
                            } else {
                                network.apply(puzzle.clone()).arg_max()
                            };

                            puzzle.apply(action).unwrap();

                            (0..P::ACTIONS_LENGTH).map(move |action| {
                                let state = state.clone();
                                let mut puzzle = state.clone();
                                puzzle.apply(action).unwrap();
                                ReplayObservation {
                                    state,
                                    action,
                                    reward: puzzle.get_reward(),
                                    next_state: puzzle,
                                }
                            })
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
            Self::Iterative {
                target_updates_per_step: _,
                instances,
                instance_replay_length,
            } => instances * instance_replay_length,
            Self::ForcedIterativeBreadth {
                target_updates_per_step: _,
                instances,
                instance_replay_length,
            }
            | Self::ForcedIterative {
                target_updates_per_step: _,
                instances,
                instance_replay_length,
            } => instances * instance_replay_length,
        }
    }
}

pub struct SampleParams {
    pub epsilon: f64,
    pub last_target_update: usize,
    pub target_update_count: usize,
    pub epoch: usize,
}
