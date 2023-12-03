use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::puzzle::{ActionOutOfBounds, Puzzle};

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct GoHome {
    x: i64,
    y: i64,
}

impl Puzzle for GoHome {
    const ACTIONS_LENGTH: usize = 8;
    const FEATURE_LENGTH: usize = 2;

    fn get_valid_actions(&self) -> Vec<usize> {
        (0..8).collect()
    }

    fn get_reward(&self) -> f64 {
        1.0 / ((self.x.abs() + self.y.abs() + 1) as f64)
    }

    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn get_features(&self) -> Vec<f64> {
        vec![
            (1.0 - 1.0 / ((self.x.abs() + 1) as f64)) * (self.x / self.x.abs()) as f64,
            (1.0 - 1.0 / ((self.y.abs() + 1) as f64)) * (self.y / self.y.abs()) as f64,
        ]
    }

    fn is_solved(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    fn apply(&mut self, action: usize) -> Result<(), crate::puzzle::ActionOutOfBounds> {
        match action {
            0 => self.y += 1,
            1 => self.y -= 1,
            2 => self.x += 1,
            3 => self.x -= 1,
            4 => {
                self.y += 1;
                self.x += 1;
            }
            5 => {
                self.y += 1;
                self.x -= 1;
            }
            6 => {
                self.y -= 1;
                self.x += 1;
            }
            7 => {
                self.y += 1;
                self.x += 1;
            }
            a => return Err(ActionOutOfBounds(a)),
        }
        Ok(())
    }
}

impl Display for GoHome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
