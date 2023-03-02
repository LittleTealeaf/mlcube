#[derive(Debug)]
pub enum ApplyActionError {
    InvalidActionIndex,
}

/// Implements functions that every puzzle should be able to manage
pub trait Puzzle: Default {
    const OBSERVATION_SIZE: usize;
    const ACTION_SIZE: usize;

    fn apply_action(&mut self, action: usize) -> Result<(), ApplyActionError>;

    fn get_observations(&self) -> Vec<u8>;

    fn reset(&mut self);

    fn is_solved(&self) -> bool;
}
