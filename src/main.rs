use puzzle::{cube::Cube2x2, Puzzle};

mod network;
mod puzzle;

fn main() {
    let mut cube = Cube2x2::new();
    println!("{:?}", cube);
    cube.apply(5).unwrap();
    println!("{:?}", cube);
    cube.apply(5).unwrap();
    println!("{:?}", cube);
    cube.apply(5).unwrap();
    println!("{:?}", cube);
    cube.apply(5).unwrap();
    println!("{:?}", cube);
}
