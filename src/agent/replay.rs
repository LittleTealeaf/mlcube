use rand::{thread_rng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{network::Network, puzzle::Puzzle, utils::ArgMax};

#[derive(Serialize, Deserialize)]
pub enum ReplayStrategy {
    EvenSample {
        scramble_depth: usize,
        instances: usize,
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
                        .into_iter()
                        .map(|_| {
                            puzzle.scramble(&mut rng).unwrap();

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
        }
    }
}

#[derive(Clone, Copy)]
pub struct ReplayObservation<P>
where
    P: Puzzle,
{
    state: P,
    action: usize,
    reward: f64,
    next_state: P,
}
