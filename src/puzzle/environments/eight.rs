use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::puzzle::{ActionOutOfBounds, Puzzle};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EightPuzzle {
    state: [usize; 9],
}

impl Puzzle for EightPuzzle {
    const ACTIONS_LENGTH: usize = 4;

    const FEATURE_LENGTH: usize = 9 * 9;

    fn new() -> Self {
        Self {
            state: [0, 1, 2, 3, 4, 5, 6, 7, 8],
        }
    }

    fn apply(&mut self, action: usize) -> Result<(), ActionOutOfBounds> {
        for i in 0..9 {
            if self.state[i] == 0 {
                match action {
                    // Down
                    0 => {
                        if i < 6 {
                            self.state.swap(i + 3, i);
                        }
                    }
                    // Up
                    1 => {
                        if i > 2 {
                            self.state.swap(i - 3, i);
                        }
                    }
                    // Right
                    2 => {
                        if i % 3 < 2 {
                            self.state.swap(i + 1, i);
                        }
                    }
                    // Left
                    3 => {
                        if i % 3 > 0 {
                            self.state.swap(i - 1, i);
                        }
                    }
                    i => Err(ActionOutOfBounds(i))?,
                }
                return Ok(());
            }
        }
        panic!("Invalid State")
    }

    fn get_features(&self) -> Vec<f64> {
        let mut features = vec![0f64; Self::FEATURE_LENGTH];
        for i in 0..9 {
            features[i + self.state[i]] = 1f64;
        }
        features
    }

    fn get_reward(&self) -> f64 {
        if self.is_solved() {
            1f64
        } else {
            0f64
        }
    }

    fn is_solved(&self) -> bool {
        for i in 0..9 {
            if self.state[i] != i {
                return false;
            }
        }
        true
    }

    fn get_valid_actions(&self) -> Vec<usize> {
        let index = self
            .state
            .iter()
            .enumerate()
            .find_map(|(index, value)| (value == &0).then_some(index))
            .unwrap();

        [
            (index < 6).then_some(0),
            (index > 2).then_some(1),
            (index % 3 < 2).then_some(2),
            (index % 3 > 0).then_some(3),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl Display for EightPuzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{} {}{}{} {}{}{}",
            self.state[0],
            self.state[1],
            self.state[2],
            self.state[3],
            self.state[4],
            self.state[5],
            self.state[6],
            self.state[7],
            self.state[8]
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerousEight(EightPuzzle);

impl Puzzle for GenerousEight {
    const ACTIONS_LENGTH: usize = EightPuzzle::ACTIONS_LENGTH;

    const FEATURE_LENGTH: usize = EightPuzzle::FEATURE_LENGTH;

    fn apply(&mut self, action: usize) -> Result<(), ActionOutOfBounds> {
        self.0.apply(action)
    }

    fn is_solved(&self) -> bool {
        self.0.is_solved()
    }

    fn get_features(&self) -> Vec<f64> {
        self.0.get_features()
    }

    fn new() -> Self {
        Self(EightPuzzle::new())
    }

    fn get_reward(&self) -> f64 {
        // Count number of correct spots
        (((0..9)
            .into_iter()
            .filter(|i| self.0.state[*i] == *i)
            .count() as f64)
            / 9.0)
            .powi(10)
    }

    fn get_valid_actions(&self) -> Vec<usize> {
        self.0.get_valid_actions()
    }
}

impl Display for GenerousEight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
