use std::fmt::Display;

use serde::{Deserialize, Serialize};

use serde_big_array::BigArray;

use crate::puzzle::{ActionOutOfBounds, Puzzle};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
struct NPuzzleRow<const W: usize>(#[serde(with = "BigArray")] [usize; W]);

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub struct NPuzzle<const H: usize, const W: usize>(#[serde(with = "BigArray")] [NPuzzleRow<W>; H]);

impl<const H: usize, const W: usize> Puzzle for NPuzzle<H, W> {
    const ACTIONS_LENGTH: usize = 4;
    const FEATURE_LENGTH: usize = H * W * H * W;

    fn new() -> Self {
        let mut state = [[0; W]; H];
        for i in 0..(H * W) {
            state[i / W][i % W] = i;
        }

        Self(state.map(NPuzzleRow))
    }

    fn apply(&mut self, action: usize) -> Result<(), crate::puzzle::ActionOutOfBounds> {
        for x in 0..W {
            for y in 0..H {
                if self.0[y].0[x] == 0 {
                    match action {
                        // Down
                        0 => {
                            if y < H - 1 {
                                (self.0[y].0[x], self.0[y + 1].0[x]) =
                                    (self.0[y + 1].0[x], self.0[y].0[x]);
                            }
                        }
                        // Up
                        1 => {
                            if y > 0 {
                                (self.0[y].0[x], self.0[y - 1].0[x]) =
                                    (self.0[y - 1].0[x], self.0[y].0[x]);
                            }
                        }
                        // Left
                        2 => {
                            if x < W - 1 {
                                (self.0[y].0[x], self.0[y].0[x + 1]) =
                                    (self.0[y].0[x + 1], self.0[y].0[x]);
                            }
                        }
                        // Right
                        3 => {
                            if x > 0 {
                                (self.0[y].0[x], self.0[y].0[x - 1]) =
                                    (self.0[y].0[x - 1], self.0[y].0[x]);
                            }
                        }

                        n => return Err(ActionOutOfBounds(n)),
                    }

                    return Ok(());
                }
            }
        }
        panic!()
    }

    fn get_features(&self) -> Vec<f64> {
        let mut features = vec![0f64; Self::FEATURE_LENGTH];
        for x in 0..W {
            for y in 0..H {
                let val = self.0[y].0[x];
                let index = y * W + x;
                features[val + W * H * index] = 1f64;
            }
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
        for i in 0..(W * H) {
            if self.0[i / W].0[i % W] != i {
                return false;
            }
        }
        true
    }

    fn get_valid_actions(&self) -> Vec<usize> {
        for y in 0..H {
            for x in 0..W {
                if self.0[y].0[x] == 0 {
                    return [
                        (y < H - 1).then_some(0),
                        (y > 0).then_some(1),
                        (x < W - 1).then_some(2),
                        (x > 0).then_some(3),
                    ]
                    .into_iter()
                    .flatten()
                    .collect();
                }
            }
        }
        panic!("Invalid State")
    }
}

impl<const H: usize, const W: usize> Display for NPuzzle<H, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..H {
            for x in 0..W {
                write!(f, "{}", self.0[y].0[x])?;
            }
            write!(f, " ")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn features() {
        let puzzle = NPuzzle::<3, 3>::new();
        puzzle.get_features();
    }
}
