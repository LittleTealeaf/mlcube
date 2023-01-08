/// Permutations for the R move
const PERMS_R: [[usize; 4]; 5] = [
    [20, 2, 42, 47],
    [23, 5, 39, 50],
    [26, 8, 36, 53],
    [27, 29, 35, 33],
    [28, 32, 34, 30],
];

/// Permutations for the U move
const PERMS_U: [[usize; 4]; 5] = [
    [20, 11, 38, 29],
    [19, 10, 37, 28],
    [18, 9, 36, 27],
    [8, 6, 0, 2],
    [7, 3, 1, 5],
];

/// Permutations for the L move
const PERMS_L: [[usize; 4]; 5] = [
    [18, 45, 44, 0],
    [21, 48, 41, 3],
    [24, 51, 38, 6],
    [11, 17, 15, 9],
    [14, 16, 12, 10],
];

/// Permutations for the D move
const PERMS_D: [[usize; 4]; 5] = [
    [24, 33, 42, 15],
    [25, 34, 43, 16],
    [26, 35, 44, 17],
    [45, 47, 53, 51],
    [46, 50, 52, 48],
];

/// Permutations for the F move
const PERMS_F: [[usize; 4]; 5] = [
    [6, 27, 47, 17],
    [7, 30, 46, 14],
    [8, 33, 45, 11],
    [18, 20, 26, 24],
    [19, 23, 25, 21],
];

/// Permutations for the B move
const PERMS_B: [[usize; 4]; 5] = [
    [36, 38, 44, 42],
    [37, 41, 43, 39],
    [29, 0, 15, 53],
    [32, 1, 12, 52],
    [35, 2, 9, 51],
];

#[derive(Clone, Copy)]
pub enum Face {
    R,
    U,
    F,
    L,
    D,
    B,
}

impl Face {
    fn from_index(index: usize) -> Result<Face, &'static str> {
        match index {
            0 => Ok(Face::U),
            1 => Ok(Face::L),
            2 => Ok(Face::F),
            3 => Ok(Face::R),
            4 => Ok(Face::B),
            5 => Ok(Face::D),
            _ => Err("Index Out of Bounds"),
        }
    }
}

impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Face::R => match other {
                Face::R => true,
                _ => false,
            },
            Face::U => match other {
                Face::U => true,
                _ => false,
            },
            Face::F => match other {
                Face::F => true,
                _ => false,
            },
            Face::L => match other {
                Face::L => true,
                _ => false,
            },
            Face::D => match other {
                Face::D => true,
                _ => false,
            },
            Face::B => match other {
                Face::B => true,
                _ => false,
            },
        }
    }
}

pub enum Move {
    Normal(Face),
    Prime(Face),
    Two(Face),
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Move::Normal(f1) => match other {
                Move::Normal(f2) => f1.eq(f2),
                _ => false,
            },
            Move::Prime(f1) => match other {
                Move::Prime(f2) => f1.eq(f2),
                _ => false,
            },
            Move::Two(f1) => match other {
                Move::Two(f2) => f1.eq(f2),
                _ => false,
            },
        }
    }
}

struct Cube {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    mod enum_face {
        use super::*;

        #[test]
        fn same_faces_are_equal() {
            assert!(Face::R.eq(&Face::R), "The same face should be equal to itself");
        }

        #[test]
        fn different_faces_are_not_equal() {
            assert!(!Face::F.eq(&Face::L), "Different faces should not be equal");
        }

        #[test]
        fn from_index_returns_a_face() {
            assert!(match Face::from_index(0) {
                Ok(_) => true,
                Err(_) => false,
            });
        }

        #[test]
        fn from_index_returns_all_faces() {
            let mut values: Vec<Face> = Vec::new();
            for i in 0..6 {
                values.push(Face::from_index(i).unwrap());
            }
            assert!(values.contains(&Face::R), "R should be accessible using the from_index method");
            assert!(values.contains(&Face::U), "U should be accessible using the from_index method");
            assert!(values.contains(&Face::F), "F should be accessible using the from_index method");
            assert!(values.contains(&Face::L), "L should be accessible using the from_index method");
            assert!(values.contains(&Face::D), "D should be accessible using the from_index method");
            assert!(values.contains(&Face::B), "B should be accessible using the from_index method");
        }
    }

    mod enum_move {
        use super::*;

        #[test]
        fn same_moves_are_equal() {
            assert!(Move::Normal(Face::R).eq(&Move::Normal(Face::R)), "Moves of the same type and face should be equal");
        }

        #[test]
        fn different_moves_are_not_equal() {
            assert!(!Move::Prime(Face::U).eq(&Move::Two(Face::U)), "Moves of different types but the same face should not be equal");
            assert!(!Move::Normal(Face::F).eq(&Move::Prime(Face::L)), "Moves of different types and different faces should not be equal");
            assert!(!Move::Two(Face::D).eq(&Move::Two(Face::B)), "Moves of the same type but different faces should not be equal");
        }
    }

    mod cube {
        use super::*;

        #[test]
        fn new_cube_has_equal_face_frequencies() {
            let mut counts = [0; 6];

            let cube = Cube::new();
            for face in cube.state {
                counts[match face {
                    Face::R => 0,
                    Face::U => 1,
                    Face::F => 2,
                    Face::D => 3,
                    Face::L => 4,
                    Face::B => 5
                }] += 1;
            }

            assert_eq!(counts[0], 9, "Frequency of Right Faces should be 9");
            assert_eq!(counts[1], 9, "Frequency of Top Faces should be 9");
            assert_eq!(counts[2], 9, "Frequency of Front Faces should be 9");
            assert_eq!(counts[3], 9, "Frequency of Bottom Faces should be 9");
            assert_eq!(counts[4], 9, "Frequency of Left Faces should be 9");
            assert_eq!(counts[5], 9, "Frequency of Back Faces should be 9");
        }
    }
}
