use crate::puzzle::{ActionOutOfBounds, Puzzle};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cube3x3([usize; 54]);

impl Puzzle for Cube3x3 {
    const ACTIONS_LENGTH: usize = 18;

    const FEATURE_LENGTH: usize = 54 * 6;

    fn new() -> Self {
        Self(SOLVED_STATE)
    }

    fn apply(&mut self, action: usize) -> Result<(), ActionOutOfBounds> {
        let permutations = PERMUTATIONS[action % 6];
        let rotation = action / 6;
        match rotation {
            0 => {
                // Normal
                for row in permutations {
                    let tmp = self.0[row[3]];
                    for i in 0..3 {
                        self.0[row[3 - i]] = self.0[row[2 - i]];
                    }
                    self.0[row[0]] = tmp;
                }

                Ok(())
            }
            1 => {
                // Prime
                for row in permutations {
                    let tmp = self.0[row[0]];
                    for i in 0..3 {
                        self.0[row[i]] = self.0[row[i + 1]];
                    }
                    self.0[row[3]] = tmp;
                }
                Ok(())
            }
            2 => {
                // Two
                for row in permutations {
                    for i in [0, 1] {
                        let tmp = self.0[row[i]];
                        self.0[row[i]] = self.0[row[i + 2]];
                        self.0[row[i + 2]] = tmp;
                    }
                }
                Ok(())
            }
            _ => Err(ActionOutOfBounds(action)),
        }
    }

    fn get_features(&self) -> Vec<f64> {
        let mut observations = [0f64; Self::FEATURE_LENGTH];

        for i in 0..54 {
            let value = self.0[i];
            observations[i * 6 + value] = 1f64;
        }

        Vec::from(observations)
    }

    fn get_reward(&self) -> f64 {
        if self.is_solved() {
            1f64
        } else {
            0f64
        }
    }

    fn is_solved(&self) -> bool {
        SOLVED_STATE.eq(&self.0)
    }

    fn get_valid_actions(&self) -> Vec<usize> {
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
    }
}

const SOLVED_STATE: [usize; 54] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5,
];

const PERMUTATIONS: [[[usize; 4]; 5]; 6] = [
    //U
    [
        [20, 11, 38, 29],
        [19, 10, 37, 28],
        [18, 9, 36, 27],
        [8, 6, 0, 2],
        [7, 3, 1, 5],
    ],
    //L
    [
        [18, 45, 44, 0],
        [21, 48, 41, 3],
        [24, 51, 38, 6],
        [11, 17, 15, 9],
        [14, 16, 12, 10],
    ],
    //F
    [
        [6, 27, 47, 17],
        [7, 30, 46, 14],
        [8, 33, 45, 11],
        [18, 20, 26, 24],
        [19, 23, 25, 21],
    ],
    //R
    [
        [20, 2, 42, 47],
        [23, 5, 39, 50],
        [26, 8, 36, 53],
        [27, 29, 35, 33],
        [28, 32, 34, 30],
    ],
    //D
    [
        [24, 33, 42, 15],
        [25, 34, 43, 16],
        [26, 35, 44, 17],
        [45, 47, 53, 51],
        [46, 50, 52, 48],
    ],
    //B
    [
        [36, 38, 44, 42],
        [37, 41, 43, 39],
        [29, 0, 15, 53],
        [32, 1, 12, 52],
        [35, 2, 9, 51],
    ],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_features_length() {
        let cube = Cube3x3::new();
        let features = cube.get_features();
        assert_eq!(Cube3x3::FEATURE_LENGTH, features.len());
    }

    #[test]
    fn new_cube_is_solved() {
        assert!(Cube3x3::new().is_solved());
    }

    #[test]
    fn valid_actions_return_ok() {
        for i in 0..Cube3x3::ACTIONS_LENGTH {
            assert!(Cube3x3::new().apply(i).is_ok());
        }
    }

    #[test]
    fn invalid_actions_return_err() {
        assert!(Cube3x3::new().apply(Cube3x3::ACTIONS_LENGTH).is_err());
    }

    #[test]
    fn apply_makes_cube_unsolved() {
        for i in 0..Cube3x3::ACTIONS_LENGTH {
            let mut cube = Cube3x3::new();
            cube.apply(i).unwrap();
            assert!(!cube.is_solved())
        }
    }

    #[test]
    fn apply_repeated_solves_cube() {
        for action in 0..Cube3x3::ACTIONS_LENGTH {
            let mut cube = Cube3x3::new();
            for _ in 0..4 {
                cube.apply(action).unwrap();
            }
            assert!(cube.is_solved())
        }
    }
}
