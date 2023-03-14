use pyo3::{pyclass, pymethods, PyResult};

use crate::puzzle::{puzzles::Cube2x2, Puzzle};

#[pyclass]
pub struct PyCube2x2 {
    cube: Cube2x2,
}

#[pymethods]
impl PyCube2x2 {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    #[classattr]
    fn observation_length() -> usize {
        Cube2x2::OBSERVATION_SIZE
    }

    fn reset(&mut self) {
        self.cube.reset();
    }

    fn is_solved(&self) -> bool {
        self.cube.is_solved()
    }

    fn get_observations(&self) -> Vec<u8> {
        self.cube.get_observations()
    }

    fn apply_action(&mut self, action: usize) -> PyResult<()> {
        self.cube.apply_action(action)?;
        Ok(())
    }

    fn scramble(&mut self, steps: usize) -> u64 {
        self.cube.scramble(steps)
    }

    fn scramble_with_seed(&mut self, seed: u64, steps: usize) {
        self.cube.scramble_with_seed(steps, seed);
    }
}

impl Default for PyCube2x2 {
    fn default() -> Self {
        Self {
            cube: Default::default(),
        }
    }
}
