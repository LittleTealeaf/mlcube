pub mod cube;

pub trait Puzzle: Clone + Eq {
    const ACTIONS_LENGTH: usize;
    const FEATURE_LENGTH: usize;

    fn new() -> Self;

    fn apply(&mut self, action: usize) -> Result<(), ActionOutOfBounds>;

    fn get_features(&self) -> Vec<f64>;

    fn get_reward(&self) -> f64;

    fn is_solved(&self) -> bool;
}

#[derive(Debug)]
pub struct ActionOutOfBounds(pub usize);
