#![allow(dead_code)]
use network::Network;
use puzzle::{eight::EightPuzzle, Puzzle};
use rand::{seq::IteratorRandom, thread_rng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use utils::ArgMax;

use crate::utils::Max;

mod network;
mod puzzle;
mod utils;

type _Puzzle = EightPuzzle;

const UPDATE_INTERVAL: usize = 100;
const MAX_SCRAMBLE_DEPTH: usize = 20;
const REPLAY_SIZE: usize = 10_000;
const PARALLEL_PUZZLES: usize = 100;
const TRAIN_SAMPLE: usize = REPLAY_SIZE / 4;

const GAMMA: f64 = 0.9;
const EPSILON: f64 = 0.3;

fn alpha(iter: usize) -> f64 {
    0.9f64.powi((iter % UPDATE_INTERVAL + 1) as i32)
}

fn main() {
    let mut network = Network::<_Puzzle>::new(vec![9 * 8 * 3, 9 * 8 * 2, 9 * 8, 9 * 8 / 2]);
    let mut rng = thread_rng();
    network.randomize(&mut rng, -0.1..0.1);

    let mut target = network.clone();

    let mut iter = 0;

    loop {
        println!("Iter {}", iter);

        let replay = (0..PARALLEL_PUZZLES)
            .into_par_iter()
            .map(|_| {
                let mut puzzle = _Puzzle::new();
                let mut rng = thread_rng();

                for _ in 0..MAX_SCRAMBLE_DEPTH {
                    puzzle
                        .apply(rng.gen_range(0.._Puzzle::ACTIONS_LENGTH))
                        .unwrap();
                }

                (0..(REPLAY_SIZE / PARALLEL_PUZZLES))
                    .into_iter()
                    .map(|_| {
                        let state = puzzle.clone();
                        let action = {
                            if rng.gen_bool(EPSILON) {
                                rng.gen_range(0.._Puzzle::ACTIONS_LENGTH)
                            } else {
                                network.apply(state).arg_max()
                            }
                        };

                        puzzle.apply(action).unwrap();

                        let expected = puzzle.get_reward() + GAMMA * target.apply(puzzle).max();

                        (state, action, expected)
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

        // let replay = (0..(REPLAY_SIZE / MAX_SCRAMBLE_DEPTH))
        //     .into_par_iter()
        //     .map(|_| {
        //         let mut puzzle = _Puzzle::new();
        //         let mut rng = thread_rng();
        //
        //         (0..MAX_SCRAMBLE_DEPTH)
        //             .into_iter()
        //             .map(|_| {
        //                 puzzle
        //                     .apply(rng.gen_range(0.._Puzzle::ACTIONS_LENGTH))
        //                     .unwrap();
        //
        //                 let state = puzzle.clone();
        //                 let action = {
        //                     if rng.gen_bool(EPSILON) {
        //                         rng.gen_range(0.._Puzzle::ACTIONS_LENGTH)
        //                     } else {
        //                         network.apply(puzzle).arg_max()
        //                     }
        //                 };
        //                 puzzle.apply(action).unwrap();
        //
        //             })
        //             .collect::<Vec<_>>()
        //     })
        //     .flatten()
        //     .collect::<Vec<_>>();

        let nudges = replay
            .into_iter()
            .choose_multiple(&mut rng, TRAIN_SAMPLE)
            .into_par_iter()
            .map(|(state, action, expected)| {
                network.back_propagate(state, action, expected, alpha(iter) / (TRAIN_SAMPLE as f64))
            })
            .reduce(
                || Vec::new(),
                |mut a, b| {
                    if a.len() == 0 {
                        return b;
                    } else if b.len() == 0 {
                        return a;
                    }
                    for i in 0..(a.len()) {
                        a[i].add(b[i].clone());
                    }
                    a
                },
            );

        network.update_weights(nudges);

        if iter % UPDATE_INTERVAL == 0 {
            target = network.clone();
        }

        let mut puzzle = _Puzzle::new();
        for _ in 0..MAX_SCRAMBLE_DEPTH {
            puzzle
                .apply(rng.gen_range(0.._Puzzle::ACTIONS_LENGTH))
                .unwrap();
        }

        let values = network.apply(puzzle);
        println!("{:?}", values);

        let result = network.solve(puzzle);

        println!("{:?}", result);

        iter += 1;
    }
}
