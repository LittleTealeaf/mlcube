
pub struct InvalidActionIndex;

pub trait Puzzle: Default {
  fn apply_action(&mut self, action: usize) -> Result<(), InvalidActionIndex>;

  fn get_observations(&self) -> Vec<u8>;

  fn reset(&mut self);

  const OBSERVATION_LENGTH: usize;
  const ACTION_SIZE: usize;

  fn is_solved(&self) -> bool;
}
