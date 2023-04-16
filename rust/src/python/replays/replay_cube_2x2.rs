use pyo3::{pyclass, pymethods, PyResult};

use crate::{
    puzzle::{puzzles::Cube2x2, Puzzle},
    replay::Replay, python::PyCube2x2,
};

use super::{sample_from_set, PyReplaySample};

type Replay2x2 = Replay<Cube2x2>;

#[pyclass]
pub struct PyReplay2x2 {
    replay: Replay2x2,
}

#[pymethods]
impl PyReplay2x2 {
    #[new]
    #[pyo3(signature = (capacity = 100_000))]
    fn new(capacity: usize) -> Self {
        Self {
            replay: Replay::with_capacity(capacity),
        }
    }

    #[classattr]
    fn observation_length() -> usize {
        Replay2x2::OBSERVATION_SIZE
    }

    #[classattr]
    fn action_size() -> usize {
        Replay2x2::ACTION_SIZE
    }

    fn apply_action(&mut self, action: usize) -> PyResult<()> {
        self.replay.apply_action(action)?;
        Ok(())
    }

    fn sample_replay(&mut self, count: usize) -> PyResult<PyReplaySample> {
        Ok(sample_from_set(self.replay.sample_replay(count)?))
    }

    fn scramble(&mut self) -> u64 {
        self.replay.scramble(100)
    }

    fn scramble_with_seed(&mut self, seed: u64) {
        self.replay.scramble_with_seed(100, seed)
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
        String::from("Cube2x2")
    }

    fn get_action_name(&self, action: usize) -> String {
        Cube2x2::get_action_name(action).unwrap_or(String::from(""))
    }

    fn create_evaluation_target(&self) -> PyCube2x2 {
        PyCube2x2::default()
    }
}

impl Default for PyReplay2x2 {
    fn default() -> Self {
        Self {
            replay: Replay::default(),
        }
    }
}
