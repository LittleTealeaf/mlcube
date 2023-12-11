use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::puzzle::{ActionOutOfBounds, Puzzle};

use serde_big_array::BigArray;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
struct LightsOutRow<const W: usize>(#[serde(with = "BigArray")] [bool; W]);

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightsOut<const H: usize, const W: usize> {
    #[serde(with = "BigArray")]
    state: [LightsOutRow<W>; H],
}

impl<const H: usize, const W: usize> Puzzle for LightsOut<H, W> {
    const ACTIONS_LENGTH: usize = H * W;

    const FEATURE_LENGTH: usize = H * W;

    fn new() -> Self {
        Self {
            state: [LightsOutRow([false; W]); H],
        }
    }

    fn apply(&mut self, action: usize) -> Result<(), crate::puzzle::ActionOutOfBounds> {
        if action >= H * W {
            return Err(ActionOutOfBounds(action));
        }

        let y = action / W;
        let x = action % W;

        #[allow(clippy::unnecessary_lazy_evaluations)]
        let mask = [
            Some((x, y)),
            (y > 0).then(|| (x, y - 1)),
            (x > 0).then(|| (x - 1, y)),
            (y < H - 1).then(|| (x, y + 1)),
            (x < W - 1).then(|| (x + 1, y)),
        ]
        .into_iter()
        .flatten();

        for (x, y) in mask {
            self.state[y].0[x] = !self.state[y].0[x];
        }

        Ok(())
    }

    fn get_features(&self) -> Vec<f64> {
        self.state
            .iter()
            .flat_map(|LightsOutRow(row)| row.iter().map(|v| if *v { 1.0 } else { 0.0 }))
            .collect()
    }

    fn get_reward(&self) -> f64 {
        if self.is_solved() {
            1f64
        } else {
            -1f64 / (W * H * 2) as f64
        }
    }

    fn is_solved(&self) -> bool {
        for LightsOutRow(row) in self.state {
            for value in row {
                if value {
                    return false;
                }
            }
        }
        true
    }

    fn get_valid_actions(&self) -> Vec<usize> {
        (0..Self::ACTIONS_LENGTH).collect()
    }
}

impl<const H: usize, const W: usize> Display for LightsOut<H, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for LightsOutRow(row) in self.state {
            for item in row {
                if item {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }
            }
            write!(f, " ")?;
        }

        Ok(())
    }
}
