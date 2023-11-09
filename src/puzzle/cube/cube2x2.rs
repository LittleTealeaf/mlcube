use crate::puzzle::{ActionOutOfBounds, Puzzle};

#[derive(Clone, Copy)]
pub struct Cube2x2([usize; 24]);

const SOLVED_STATE: [usize; 24] = [
    0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5,
];

const PERMUTATIONS: [[[usize; 4]; 3]; 3] = [
    // U
    [[0, 1, 3, 2], [8, 4, 16, 12], [9, 5, 17, 13]],
    // F
    [[8, 9, 11, 10], [5, 3, 14, 20], [7, 2, 12, 21]],
    // R
    [[14, 12, 13, 15], [9, 1, 18, 21], [11, 3, 16, 23]],
];

impl Puzzle for Cube2x2 {
    const ACTIONS_LENGTH: usize = 6;
    const FEATURE_LENGTH: usize = 4 * 6 * 6;

    fn solved() -> Self {
        Self(SOLVED_STATE)
    }

    fn apply(&mut self, action: usize) -> Result<(), crate::puzzle::ActionOutOfBounds> {
        let permutations = PERMUTATIONS[action % 3];
        let rotation = action / 3;
        match rotation {
            0 => {
                // Normal
                for row in permutations {
                    let tmp = self.0[row[3]];
                    self.0[row[3]] = self.0[row[2]];
                    self.0[row[2]] = self.0[row[1]];
                    self.0[row[1]] = self.0[row[0]];
                    self.0[row[0]] = tmp;
                }
                Ok(())
            }
            1 => {
                // Reverse
                for row in permutations {
                    let tmp = self.0[row[0]];
                    self.0[row[0]] = self.0[row[1]];
                    self.0[row[1]] = self.0[row[2]];
                    self.0[row[2]] = self.0[row[3]];
                    self.0[row[3]] = tmp;
                }

                Ok(())
            }
            2 => {
                // Double
                for row in permutations {
                    (self.0[row[0]], self.0[row[2]]) = (self.0[row[2]], self.0[row[0]]);
                    (self.0[row[1]], self.0[row[3]]) = (self.0[row[3]], self.0[row[1]]);
                }
                Ok(())
            }
            _ => Err(ActionOutOfBounds(action)),
        }
    }

    fn get_features(&self) -> Vec<f64> {
        let mut features = vec![0f64; Self::FEATURE_LENGTH];

        for (index, value) in self.0.iter().enumerate() {
            features[index * 6 + *value] = 1f64;
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
        for i in 0..24 {
            if self.0[i] != i / 4 {
                return false;
            }
        }
        return true;
    }
}
