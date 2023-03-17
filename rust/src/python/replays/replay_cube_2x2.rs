use pyo3::{pyclass, pymethods, PyResult};

use crate::{puzzle::puzzles::Cube2x2, replay::Replay};

use super::PyReplayEntry;

#[pyclass]
pub struct PyReplay2x2 {
    replay: Replay<Cube2x2>,
}

#[pymethods]
impl PyReplay2x2 {
    #[new]
    fn new(capacity: usize) -> Self {
        Self {
            replay: Replay::with_capacity(capacity),
        }
    }

    fn record_action(&mut self, action: usize, reward: f64) -> PyResult<()> {
        self.replay.record_action(action, reward)?;
        Ok(())
    }

    fn sample_replay(&mut self, count: usize) -> Vec<PyReplayEntry> {
        self.replay
            .sample_replay(count)
            .into_iter()
            .map(PyReplayEntry::from)
            .collect()
    }
}

impl Default for PyReplay2x2 {
    fn default() -> Self {
        Self {
            replay: Replay::default(),
        }
    }
}
