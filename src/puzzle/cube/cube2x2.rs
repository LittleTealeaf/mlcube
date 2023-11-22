use crate::puzzle::{ActionOutOfBounds, Puzzle};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    const ACTIONS_LENGTH: usize = 9;
    const FEATURE_LENGTH: usize = 4 * 6 * 6;

    fn new() -> Self {
        Self(SOLVED_STATE)
    }

    fn apply(&mut self, action: usize) -> Result<(), ActionOutOfBounds> {
        let permutations = PERMUTATIONS[action % 3];
        match action / 3 {
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
        true
    }

    fn get_valid_actions(&self) -> Vec<usize> {
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_features_length() {
        let cube = Cube2x2::new();
        let features = cube.get_features();
        assert_eq!(Cube2x2::FEATURE_LENGTH, features.len());
    }

    #[test]
    fn new_cube_is_solved() {
        assert!(Cube2x2::new().is_solved());
    }

    #[test]
    fn valid_actions_return_ok() {
        for i in 0..Cube2x2::ACTIONS_LENGTH {
            assert!(Cube2x2::new().apply(i).is_ok());
        }
    }

    #[test]
    fn invalid_actions_return_err() {
        assert!(Cube2x2::new().apply(Cube2x2::ACTIONS_LENGTH).is_err());
    }

    #[test]
    fn apply_makes_cube_unsolved() {
        for i in 0..Cube2x2::ACTIONS_LENGTH {
            let mut cube = Cube2x2::new();
            cube.apply(i).unwrap();
            assert!(!cube.is_solved())
        }
    }

    #[test]
    fn apply_repeated_solves_cube() {
        for action in 0..Cube2x2::ACTIONS_LENGTH {
            let mut cube = Cube2x2::new();
            for _ in 0..4 {
                cube.apply(action).unwrap();
            }
            assert!(cube.is_solved())
        }
    }
}
