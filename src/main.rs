#![allow(dead_code)]
use network::Network;
use puzzle::Puzzle;
use rand::{thread_rng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use utils::ArgMax;

use crate::{puzzle::cube::Cube2x2, utils::Max};

mod network;
mod puzzle;
mod utils;

fn main() {
    let mut network = Network::<Cube2x2>::new(vec![100, 100, 100, 100]);
    let mut rng = thread_rng();
    network.randomize(&mut rng);

    const CUBE_COUNT: usize = 12;

    const REPLAY_SIZE: usize = CUBE_COUNT * 200;

    let mut target = network.clone();
    let mut update_count = 0;

    let mut iter = 0;

    loop {
        iter += 1;
        println!("Start Iter {}", iter);

        let replay = vec![Cube2x2::new(); CUBE_COUNT]
            .into_par_iter()
            .map(|mut cube| {
                let mut rng = thread_rng();

                for _ in 0..10 {
                    cube.apply(rng.gen_range(0..Cube2x2::ACTIONS_LENGTH))
                        .unwrap();
                }

                let mut replay = Vec::new();
                for _ in 0..(REPLAY_SIZE / 12) {
                    let state = cube.clone();
                    let action = {
                        if rng.gen_bool(0.8) {
                            rng.gen_range(0..Cube2x2::ACTIONS_LENGTH)
                        } else {
                            network.apply(cube).arg_max()
                        }
                    };
                    cube.apply(action).unwrap();
                    let expected = 0.3 * cube.get_reward() + 0.7 * target.apply(cube).max();
                    replay.push((state, action, expected));
                }
                replay
            })
            .flatten()
            .collect::<Vec<_>>();

        println!("Created Replay");

        let nudges = replay
            .into_par_iter()
            .map(|(state, action, expected)| {
                network.back_propagate(state, action, expected, 0.01 / (REPLAY_SIZE as f64))
            })
            .collect::<Vec<_>>();

        println!("Created Nudges");

        for nudge in nudges {
            network.update_weights(nudge);
        }

        println!("Updated Nudges");

        update_count += 1;
        if update_count == 10 {
            target = network.clone();
            update_count = 0;
        }
    }
}
