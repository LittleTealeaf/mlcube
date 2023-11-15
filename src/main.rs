#![allow(dead_code)]
use network::Network;
use puzzle::Puzzle;
use rand::{seq::IteratorRandom, thread_rng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use utils::ArgMax;

use crate::{puzzle::cube::Cube2x2, utils::Max};

mod network;
mod puzzle;
mod utils;

fn main() {
    let mut network = Network::<Cube2x2>::new(vec![300, 200, 200, 100, 100]);
    let mut rng = thread_rng();
    network.randomize(&mut rng, -0.1..0.1);

    const UPDATE_INTERVAL: usize = 50;
    const MAX_SCRAMBLE_DEPTH: usize = 20;
    const REPLAY_SIZE: usize = 2_000;
    const TRAIN_SAMPLE: usize = REPLAY_SIZE * 3 / 4;

    let mut target = network.clone();
    let mut update_count = 0;

    let mut iter = 0;

    loop {
        iter += 1;
        println!("Iter {}", iter);

        let replay = (0..(REPLAY_SIZE / MAX_SCRAMBLE_DEPTH))
            .into_par_iter()
            .map(|_| {
                let mut cube = Cube2x2::new();
                let mut rng = thread_rng();

                (0..MAX_SCRAMBLE_DEPTH)
                    .into_iter()
                    .map(|_| {
                        cube.apply(rng.gen_range(0..Cube2x2::ACTIONS_LENGTH))
                            .unwrap();

                        let state = cube.clone();
                        let action = {
                            if rng.gen_bool(0.4) {
                                rng.gen_range(0..Cube2x2::ACTIONS_LENGTH)
                            } else {
                                network.apply(cube).arg_max()
                            }
                        };
                        cube.apply(action).unwrap();

                        let expected = if cube.is_solved() {
                            cube.get_reward() * 2f64
                        } else {
                            cube.get_reward() + 0.8 * target.apply(cube).max()
                        };
                        (state, action, expected)
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

        let nudges = replay
            .into_iter()
            .choose_multiple(&mut rng, TRAIN_SAMPLE)
            .into_par_iter()
            .map(|(state, action, expected)| {
                network.back_propagate(
                    state,
                    action,
                    expected,
                    0.9f64.powi((iter % UPDATE_INTERVAL + iter / UPDATE_INTERVAL * 2) as i32)
                        / (TRAIN_SAMPLE as f64),
                )
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

        if update_count == UPDATE_INTERVAL {
            target = network.clone();
            update_count = 0;
        }
        update_count += 1;

        let mut cube = Cube2x2::new();
        for _ in 0..MAX_SCRAMBLE_DEPTH {
            cube.apply(rng.gen_range(0..Cube2x2::ACTIONS_LENGTH))
                .unwrap();
        }

        let values = network.apply(cube);
        println!("{:?}", values);

        let result = network.solve(cube);

        println!("{:?}", result);
    }
}
