#![allow(dead_code)]
use puzzle::Puzzle;

use crate::puzzle::cube::Cube3x3;
mod puzzle;

fn main() {
    let mut cube = Cube3x3::new();

    for i in 0..18 {
        cube.apply(i).unwrap();
        println!("{:?}", cube);
    }
}
