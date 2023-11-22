use serde::{Serialize, Deserialize};


pub mod cube;
pub mod eight;

pub trait Puzzle: Clone + Eq + Serialize {
    /// Total number of unique actions that can be taken on this puzzle.
    ///
    /// It is possible that a puzzle may not have all actions availiable at all states. To get
    /// curently valid actions, check [`Puzzle::get_valid_actions`]
    const ACTIONS_LENGTH: usize;
    /// Vector length of features outputted by this puzzle
    const FEATURE_LENGTH: usize;

    /// Creates a new, solved, position of the puzzle
    fn new() -> Self;

    fn apply(&mut self, action: usize) -> Result<(), ActionOutOfBounds>;

    fn get_features(&self) -> Vec<f64>;

    fn get_reward(&self) -> f64;

    fn is_solved(&self) -> bool;

    fn get_valid_actions(&self) -> Vec<usize>;
}

#[derive(Debug)]
pub struct ActionOutOfBounds(pub usize);
