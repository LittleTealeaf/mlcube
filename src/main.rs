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
    let mut network = Network::<Cube2x2>::new(vec![100, 50, 25]);
    let mut rng = thread_rng();
    network.randomize(&mut rng);

    const REPLAY_SIZE: usize = 100;

    let mut target = network.clone();
    let mut update_count = 0;

    loop {
        let mut cube = Cube2x2::new();
        // for _ in 0..100 {
        //     cube.apply(rng.gen_range(0..Cube2x2::ACTIONS_LENGTH))
        //         .unwrap();
        // }

        let mut replay = Vec::with_capacity(REPLAY_SIZE);

        for _ in 0..REPLAY_SIZE {
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

        let nudges = replay
            .into_par_iter()
            .map(|(state, action, expected)| {
                network.back_propagate(state, action, expected, 0.01 / (REPLAY_SIZE as f64))
            })
            .collect::<Vec<_>>();

        for nudge in nudges {
            network.update_weights(nudge);
        }

        update_count += 1;
        if update_count == 10 {
            target = network.clone();
            update_count = 0;

            for _ in 0..10 {
                println!("{:?}", cube);
                let values = network.apply(cube);
                println!("{:?}", values);
                let action = values.arg_max();
                println!("Action: {}, Reward {}", action, cube.get_reward());
                cube.apply(action).unwrap();
                println!("New Reward: {}", cube.get_reward());
            }
        }
    }

    // for i in 0..10000 {
    //     let prev = cube.clone();
    //
    //     let action = {
    //         if rng.gen_bool(0.5) {
    //             rng.gen_range(0..Cube2x2::ACTIONS_LENGTH)
    //         } else {
    //             network.apply(cube).arg_max()
    //         }
    //     };
    //
    //     cube.apply(action).unwrap();
    //
    //     let expected = prev.get_reward() + network.apply(cube).max() * 0.8 + 0.1;
    //     network.update_weights(network.back_propagate(prev, action, expected, 0.01));
    //
    //     println!("{:?} {} {}", network.apply(prev), action, cube.get_reward());
    // }
}
