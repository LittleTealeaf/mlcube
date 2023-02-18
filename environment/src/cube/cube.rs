use super::{face::Face, Action};
pub struct Cube {
    state: [Face; 9 * 6],
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            state: get_initial_state(),
        }
    }

    pub fn apply_action(&mut self, action: usize) -> Option<()> {
        let face = Face::from_index(action / 3)?;
        let action = Action::from_index(action % 3)?;

        let permutations = get_permutations(&face);

        match action {
            Action::Normal => {
                for row in permutations {
                    let tmp = self.state[row[3]];
                    for i in 0..3 {
                        self.state[row[3 - i]] = self.state[row[2 - i]];
                    }
                    self.state[row[0]] = tmp;
                }
            }
            Action::Prime => {
                for row in permutations {
                    let tmp = self.state[row[0]];
                    for i in 0..3 {
                        self.state[row[i]] = self.state[row[i + 1]];
                    }
                    self.state[row[3]] = tmp;
                }
            }
            Action::Two => {
                for row in permutations {
                    for i in [0, 1] {
                        let tmp = self.state[row[i]];
                        self.state[row[i]] = self.state[row[i + 2]];
                        self.state[row[i + 2]] = tmp;
                    }
                }
            }
        }

        Some(())
    }

    pub fn get_observations(&self) -> [u8; 54 * 6] {
        let mut values = [0; 54 * 6];

        for i in 0..54 {
            values[i * 6 + self.state[i].to_index()] = 1;
        }

        values
    }

    pub fn reset(&mut self) {
        self.state = get_initial_state();
    }
}

fn get_permutations(face: &Face) -> [[usize; 4]; 5] {
    match face {
        Face::R => [
            [20, 2, 42, 47],
            [23, 5, 39, 50],
            [26, 8, 36, 53],
            [27, 29, 35, 33],
            [28, 32, 34, 30],
        ],
        Face::U => [
            [20, 11, 38, 29],
            [19, 10, 37, 28],
            [18, 9, 36, 27],
            [8, 6, 0, 2],
            [7, 3, 1, 5],
        ],
        Face::F => [
            [6, 27, 47, 17],
            [7, 30, 46, 14],
            [8, 33, 45, 11],
            [18, 20, 26, 24],
            [19, 23, 25, 21],
        ],
        Face::L => [
            [18, 45, 44, 0],
            [21, 48, 41, 3],
            [24, 51, 38, 6],
            [11, 17, 15, 9],
            [14, 16, 12, 10],
        ],
        Face::D => [
            [24, 33, 42, 15],
            [25, 34, 43, 16],
            [26, 35, 44, 17],
            [45, 47, 53, 51],
            [46, 50, 52, 48],
        ],
        Face::B => [
            [36, 38, 44, 42],
            [37, 41, 43, 39],
            [29, 0, 15, 53],
            [32, 1, 12, 52],
            [35, 2, 9, 51],
        ],
    }
}

fn get_initial_state() -> [Face; 54] {
    [
        Face::U,
        Face::U,
        Face::U,
        Face::U,
        Face::U,
        Face::U,
        Face::U,
        Face::U,
        Face::U,
        Face::L,
        Face::L,
        Face::L,
        Face::L,
        Face::L,
        Face::L,
        Face::L,
        Face::L,
        Face::L,
        Face::F,
        Face::F,
        Face::F,
        Face::F,
        Face::F,
        Face::F,
        Face::F,
        Face::F,
        Face::F,
        Face::R,
        Face::R,
        Face::R,
        Face::R,
        Face::R,
        Face::R,
        Face::R,
        Face::R,
        Face::R,
        Face::B,
        Face::B,
        Face::B,
        Face::B,
        Face::B,
        Face::B,
        Face::B,
        Face::B,
        Face::B,
        Face::D,
        Face::D,
        Face::D,
        Face::D,
        Face::D,
        Face::D,
        Face::D,
        Face::D,
        Face::D,
    ]
}