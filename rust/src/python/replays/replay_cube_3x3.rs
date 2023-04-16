use pyo3::{pyclass, pymethods, PyResult};

use crate::{
    puzzle::{puzzles::Cube3x3, Puzzle},
    replay::Replay, python::PyCube3x3,
};

use super::{PyReplaySample, sample_from_set};

type Replay3x3 = Replay<Cube3x3>;

#[pyclass]
pub struct PyReplay3x3 {
    replay: Replay3x3,
}

#[pymethods]
impl PyReplay3x3 {
    #[new]
    #[pyo3(signature = (capacity = 100_000))]
    fn new(capacity: usize) -> Self {
        Self {
            replay: Replay::with_capacity(capacity),
        }
    }

    #[classattr]
    fn observation_length() -> usize {
        Replay3x3::OBSERVATION_SIZE
    }

    #[classattr]
    fn action_size() -> usize {
        Replay3x3::ACTION_SIZE
    }

    fn apply_action(&mut self, action: usize) -> PyResult<()> {
        self.replay.apply_action(action)?;
        Ok(())
    }

    fn sample_replay(&mut self, count: usize) -> PyResult<PyReplaySample> {
        Ok(sample_from_set(self.replay.sample_replay(count)?))
    }

    fn scramble(&mut self, steps: usize) -> u64 {
        self.replay.scramble(steps)
    }

    fn scramble_with_seed(&mut self, steps: usize, seed: u64) {
        self.replay.scramble_with_seed(steps, seed)
    }

    fn reset(&mut self) {
        self.replay.reset()
    }

    fn is_solved(&mut self) -> bool {
        self.replay.is_solved()
    }

    fn is_at_capacity(&self) -> bool {
        self.replay.is_at_capacity()
    }

    fn get_observations(&self) -> Vec<u8> {
        self.replay.get_observations()
    }

    fn get_reward(&self) -> f64 {
        self.replay.get_reward()
    }

    fn get_name(&self) -> String {
        String::from("Cube3x3")
    }

    fn get_action_name(&self, action: usize) -> String {
        Cube3x3::get_action_name(action).unwrap_or(String::from(""))
    }

    fn create_evaluation_target(&self) -> PyCube3x3 {
        PyCube3x3::default()
    }
}
