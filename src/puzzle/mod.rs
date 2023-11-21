pub mod cube;
pub mod eight;

pub trait Puzzle: Clone + Eq {
    const ACTIONS_LENGTH: usize;
    const FEATURE_LENGTH: usize;

    fn new() -> Self;

    fn apply(&mut self, action: usize) -> Result<(), ActionOutOfBounds>;

    fn get_features(&self) -> Vec<f64>;

    fn get_reward(&self) -> f64;

    fn is_solved(&self) -> bool;

    fn get_valid_actions(&self) -> Vec<usize>;

}

#[derive(Debug)]
pub struct ActionOutOfBounds(pub usize);
