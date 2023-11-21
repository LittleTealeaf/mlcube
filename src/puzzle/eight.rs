use super::{ActionOutOfBounds, Puzzle};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EightPuzzle([usize; 9]);

impl Puzzle for EightPuzzle {
    const ACTIONS_LENGTH: usize = 4;

    const FEATURE_LENGTH: usize = 9 * 8;

    fn new() -> Self {
        Self([0, 1, 2, 3, 4, 5, 6, 7, 8])
    }

    fn apply(&mut self, action: usize) -> Result<(), super::ActionOutOfBounds> {
        for i in 0..9 {
            if self.0[i] == 0 {
                match action {
                    // Down
                    0 => {
                        if i < 6 {
                            (self.0[i + 3], self.0[i]) = (self.0[i], self.0[i + 3]);
                        }
                    }
                    // Up
                    1 => {
                        if i > 2 {
                            (self.0[i - 3], self.0[i]) = (self.0[i], self.0[i - 3]);
                        }
                    }
                    // Right
                    2 => {
                        if i % 3 < 2 {
                            (self.0[i + 1], self.0[i]) = (self.0[i], self.0[i + 1]);
                        }
                    }
                    // Left
                    3 => {
                        if i % 3 > 0 {
                            (self.0[i - 1], self.0[i]) = (self.0[i], self.0[i - 1]);
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
            if self.0[i] != 0 {
                features[i + self.0[i] - 1] = 1f64;
            }
        }
        features
    }

    fn get_reward(&self) -> f64 {
        let mut sum = 0f64;
        for i in 0..9 {
            if self.0[i] == i {
                sum += 1f64;
            }
        }

        sum * sum
    }

    fn is_solved(&self) -> bool {
        for i in 0..9 {
            if self.0[i] != i {
                return false
            }
        }
        return true
    }
}
