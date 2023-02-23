use super::{Face, InvalidFaceIndex, InvalidRotationIndex, Rotation};

pub struct Action {
    face: Face,
    rotation: Rotation,
}

impl Action {
    fn get_face(&self) -> &Face {
        &self.face
    }

    fn get_rotation(&self) -> &Rotation {
        &self.rotation
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
