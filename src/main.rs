#![allow(dead_code)]
use network::Network;
use puzzle::Puzzle;
use rand::{thread_rng, Rng};
use utils::ArgMax;

use crate::puzzle::cube::Cube2x2;

mod network;
mod puzzle;
mod utils;

fn main() {
    let mut network =
        Network::<Cube2x2>::new(vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100]);
    let mut rng = thread_rng();
    network.randomize(&mut rng);
    let mut cube = Cube2x2::new();

    for i in 0..Cube2x2::ACTIONS_LENGTH {
        cube.apply(i).unwrap();
    }

    for _ in 0..1000 {
        let rand = rng.gen_bool(0.5);

        if rand {
            let value = rng.gen_range(0..Cube2x2::ACTIONS_LENGTH);
            cube.apply(value).unwrap();
        } else {
            let values = network.apply(cube);
            let i = values.arg_max();
            cube.apply(i).unwrap();
        }
    }
}
