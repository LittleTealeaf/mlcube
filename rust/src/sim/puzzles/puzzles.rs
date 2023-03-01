use crate::sim::{Puzzle, PuzzleTrait};

use super::{Cube2x2, Cube3x3};

pub enum Puzzles {
    Cube2x2(Puzzle<Cube2x2>),
    Cube3x3(Puzzle<Cube3x3>),
}

impl From<Puzzle<Cube2x2>> for Puzzles {
    fn from(value: Puzzle<Cube2x2>) -> Self {
        Self::Cube2x2(value)
    }
}

impl From<Puzzle<Cube3x3>> for Puzzles {
    fn from(value: Puzzle<Cube3x3>) -> Self {
        Self::Cube3x3(value)
    }
}
