use std::collections::HashSet;

use pyo3::{pyclass, pymethods, PyResult};

use crate::puzzle::{puzzles::Cube3x3, Puzzle};

#[pyclass]
pub struct PyCube3x3 {
    cube: Cube3x3,
    prior_states: HashSet<[usize; 54]>,
}

#[pymethods]
impl PyCube3x3 {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    #[classattr]
    fn observation_length() -> usize {
        Cube3x3::OBSERVATION_SIZE
    }

    #[classattr]
    fn action_size() -> usize {
        Cube3x3::ACTION_SIZE
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
        String::from("Cube3x3")
    }

    fn get_action_name(&self, action: usize) -> String {
        Cube3x3::get_action_name(action).unwrap_or(String::from(""))
    }
}

impl Default for PyCube3x3 {
    fn default() -> Self {
        Self {
            cube: Default::default(),
            prior_states: HashSet::new(),
        }
    }
}
