use super::{Face, InvalidFaceIndex, InvalidRotationIndex, Rotation};

pub struct Action {
    pub face: Face,
    pub rotation: Rotation,
}

impl Action {
    pub fn new(face: Face, rotation: Rotation) -> Self {
        Self { face, rotation }
    }

    pub fn get_permutations(&self) -> [[usize; 4]; 5] {
        match self.face {
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
}

pub struct InvalidActionIndex;

impl From<InvalidFaceIndex> for InvalidActionIndex {
    fn from(_: InvalidFaceIndex) -> Self {
        InvalidActionIndex
    }
}

impl From<InvalidRotationIndex> for InvalidActionIndex {
    fn from(_: InvalidRotationIndex) -> Self {
        InvalidActionIndex
    }
}

impl TryFrom<usize> for Action {
    type Error = InvalidActionIndex;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self {
            face: Face::try_from(value / 3)?,
            rotation: Rotation::try_from(value % 3)?,
        })
    }
}

impl From<Action> for usize {
    fn from(val: Action) -> Self {
        usize::from(val.face) * 3 + usize::from(val.rotation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_index_correct() {
        let moves = [
            (0, Face::U, Rotation::Normal),
            (1, Face::U, Rotation::Prime),
            (2, Face::U, Rotation::Two),
            (3, Face::L, Rotation::Normal),
            (4, Face::L, Rotation::Prime),
            (5, Face::L, Rotation::Two),
            (6, Face::F, Rotation::Normal),
            (7, Face::F, Rotation::Prime),
            (8, Face::F, Rotation::Two),
            (9, Face::R, Rotation::Normal),
            (10, Face::R, Rotation::Prime),
            (11, Face::R, Rotation::Two),
            (12, Face::B, Rotation::Normal),
            (13, Face::B, Rotation::Prime),
            (14, Face::B, Rotation::Two),
            (15, Face::D, Rotation::Normal),
            (16, Face::D, Rotation::Prime),
            (17, Face::D, Rotation::Two),
        ];

        for (index, face, rotation) in moves {
            assert_eq!(usize::from(Action { face, rotation }), index);
        }
    }

    #[test]
    fn from_index_correct() {
        for i in 0..18 {
            let face = Action::try_from(i);
            let condition = match face {
                Ok(face) => i == usize::from(face),
                Err(_) => false,
            };
            assert!(condition);
        }
    }

    #[test]
    fn from_invalid_index_returns_error() {
        let result = Rotation::try_from(14);
        assert!(match result {
            Ok(_) => false,
            Err(_) => true,
        })
    }
}
