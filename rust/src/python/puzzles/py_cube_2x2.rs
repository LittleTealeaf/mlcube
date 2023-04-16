use std::collections::HashSet;

use pyo3::{pyclass, pymethods, PyResult};

use crate::puzzle::{puzzles::Cube2x2, Puzzle};

/// A PyCube3x3
#[pyclass]
pub struct PyCube2x2 {
    cube: Cube2x2,
    prior_states: HashSet<[usize; 24]>,
}

#[pymethods]
impl PyCube2x2 {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    /// The length of the array returned from `get_observations()`
    #[classattr]
    fn observation_length() -> usize {
        Cube2x2::OBSERVATION_SIZE
    }

    #[classattr]
    fn action_size() -> usize {
        Cube2x2::ACTION_SIZE
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
        self.prior_states.insert(self.cube.get_state());
        self.cube.apply_action(action)?;
        Ok(())
    }

    fn has_looped(&self) -> bool {
        self.prior_states.contains(&self.cube.get_state())
    }

    fn clear_prior_states(&mut self) {
        self.prior_states.clear();
    }

    fn scramble(&mut self, steps: usize) -> u64 {
        self.cube.scramble(steps)
    }

    fn scramble_with_seed(&mut self, seed: u64, steps: usize) {
        self.cube.scramble_with_seed(steps, seed);
    }

    fn get_reward(&self) -> f64 {
        self.cube.get_reward()
    }

    fn get_name(&self) -> String {
        String::from("Cube2x2")
    }

    fn get_action_name(&self, action: usize) -> String {
        Cube2x2::get_action_name(action).unwrap_or(String::from(""))
    }
}

impl Default for PyCube2x2 {
    fn default() -> Self {
        Self {
            cube: Default::default(),
            prior_states: HashSet::new(),
        }
    }
}
