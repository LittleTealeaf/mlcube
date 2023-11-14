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
    let mut network = Network::<Cube2x2>::new(vec![144, 100, 88, 50, 30, 20]);
    let mut rng = thread_rng();
    network.randomize(&mut rng);

    const UPDATE_INTERVAL: usize = 20;
    const CUBE_COUNT: usize = 12;
    const REPLAY_SIZE: usize = CUBE_COUNT * 1000;

    let mut target = network.clone();
    let mut update_count = UPDATE_INTERVAL;

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
                        if rng.gen_bool(0.3) {
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
            .into_iter()
            .choose_multiple(&mut rng, (REPLAY_SIZE * 10) / 100)
            .into_par_iter()
            .map(|(state, action, expected)| {
                network.back_propagate(
                    state,
                    action,
                    expected,
                    (1f64 / (iter as f64 + 0.5f64)) / (REPLAY_SIZE as f64),
                )
            })
            .collect::<Vec<_>>();

        println!("Created Nudges");

        for nudge in nudges {
            network.update_weights(nudge);
        }

        println!("Updated Nudges");

        if update_count == UPDATE_INTERVAL {
            target = network.clone();
            update_count = 0;
        }
        update_count += 1;

        let mut cube = Cube2x2::new();
        for _ in 0..100 {
            cube.apply(rng.gen_range(0..Cube2x2::ACTIONS_LENGTH))
                .unwrap();
        }

        for _ in 0..5 {
            let values = network.apply(cube);
            println!("{:?}", values);
            let choice = values.arg_max();
            cube.apply(choice).unwrap();
        }
        // let values = network.apply(cube);
        // println!("{:?}", values);
        // println!("State with Reward {}, chose {} with estimated utility {}, ended in state with reward {}", state.get_reward(), choice, value, cube.get_reward());
    }
}
