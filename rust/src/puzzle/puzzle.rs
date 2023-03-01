

#[derive(Debug)]
pub struct InvalidActionIndex;

pub trait Puzzle: Default {
    const OBSERVATION_SIZE: usize;

    fn apply_action(&mut self, action: usize) -> Result<(), InvalidActionIndex>;
    
    fn get_observations(&self) -> Vec<u8>;

    fn reset(&mut self);

    fn is_solved(&self) -> bool;
}
