use rand::Rng;

use super::{action::Action, face::Face};

#[derive(Clone, Copy, Hash)]
pub struct Cube {
    state: [Face; 9 * 6],
}

impl Cube {
    fn new() -> Cube {
        let mut state = [Face::U; 9 * 6];
        for i in 1..6 {
            for j in 0..9 {
                state[i * 9 + j] = Face::from_index(i).unwrap();
            }
        }

        Cube { state }
    }

    fn is_solved(&self) -> bool {
        for i in 0..54 {
            let face = self.state[i];
            if i / 9 != face.to_index() {
                return false;
            }
        }
        true
    }

    fn reset(&mut self) {
        for i in 0..54 {
            self.state[i] = Face::from_index(i / 9).unwrap();
        }
    }

    fn apply_move(&mut self, action: &Action) {
        match action {
            Action::Normal(face) => {
                for row in get_permutations(face) {
                    let tmp = self.state[row[3]];
                    for i in 0..3 {
                        self.state[row[3 - i]] = self.state[row[2 - i]]
                    }
                    self.state[row[0]] = tmp;
                }
            }
            Action::Prime(face) => {
                for row in get_permutations(face) {
                    let tmp = self.state[row[0]];
                    for i in 0..3 {
                        self.state[row[i]] = self.state[row[i + 1]];
                    }
                    self.state[row[3]] = tmp;
                }
            }
            Action::Two(face) => {
                for row in get_permutations(face) {
                    for i in [0, 1] {
                        let tmp = self.state[row[i]];
                        self.state[row[i]] = self.state[row[i + 2]];
                        self.state[row[i + 2]] = tmp;
                    }
                }
            }
        }
    }

    fn scramble(&mut self, count: u32) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let action = Action::from_value(rng.gen_range(0..18)).unwrap();
            self.apply_move(&action);
        }
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..54 {
            if self.state[i].ne(&other.state[i]) {
                return false;
            }
        }
        true
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cube_is_solved() {
        let cube = Cube::new();
        assert!(cube.is_solved());
    }

    #[test]
    fn any_move_unsolves_cube() {
        for i in 0..18 {
            let action = Action::from_value(i).unwrap();
            let mut cube = Cube::new();
            cube.apply_move(&action);
            assert!(!cube.is_solved());
        }
    }

    #[test]
    fn repeated_moves_loops_itself() {
        for i in 0..18 {
            let action = Action::from_value(i).unwrap();
            let mut cube = Cube::new();
            for _ in 0..4 {
                cube.apply_move(&action);
                if cube.is_solved() {
                    break;
                }
            }
            assert!(cube.is_solved());
        }
    }
}
