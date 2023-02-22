pub enum Face {
    R,
    U,
    F,
    L,
    D,
    B,
}

pub struct InvalidFaceIndex;

impl TryFrom<usize> for Face {
    type Error = InvalidFaceIndex;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::R),
            1 => Ok(Self::U),
            2 => Ok(Self::F),
            3 => Ok(Self::L),
            4 => Ok(Self::D),
            5 => Ok(Self::B),
            _ => Err(InvalidFaceIndex),
        }
    }
}

impl From<Face> for usize {
    fn from(val: Face) -> Self {
        match val {
            Face::R => 0,
            Face::U => 1,
            Face::F => 2,
            Face::L => 3,
            Face::D => 4,
            Face::B => 5,
        }
    }
}
