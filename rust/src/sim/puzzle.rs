use std::marker::PhantomData;

pub struct Puzzle<Size> {
    pub(crate) state: Vec<usize>,
    pub(crate) size: PhantomData<Size>,
}

pub struct InvalidActionIndex;

pub trait PuzzleTrait<Size>: Default {
    const OBSERVATION_LENGTH: usize;
    const ACTION_SIZE: usize;

    fn apply_action(&mut self, action: usize) -> Result<(), InvalidActionIndex>;

    fn get_observations(&self) -> Vec<u8>;

    fn reset(&mut self);

    fn is_solved(&self) -> bool;
}
