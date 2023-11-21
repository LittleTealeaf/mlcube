use rand::{rngs::ThreadRng, Rng};

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

    fn scramble(&mut self, rng: &mut ThreadRng) -> Result<(), ActionOutOfBounds> {
        self.apply(rng.gen_range(0..Self::ACTIONS_LENGTH))
    }

}

#[derive(Debug)]
pub struct ActionOutOfBounds(pub usize);
