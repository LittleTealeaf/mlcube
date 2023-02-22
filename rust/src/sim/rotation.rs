use crate::traits::Indexable;

pub enum Rotation {
    Normal,
    Prime,
    Two,
}

impl Indexable<usize, Self> for Rotation {
    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Normal),
            1 => Some(Self::Prime),
            2 => Some(Self::Two),
            _ => None,
        }
    }

    fn to_index(&self) -> usize {
        match self {
            Self::Normal => 0,
            Self::Prime => 1,
            Self::Two => 2,
        }
    }
}

pub struct InvalidRotationIndex;

impl TryFrom<usize> for Rotation {
    type Error = InvalidRotationIndex;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Prime),
            2 => Ok(Self::Two),
            _ => Err(InvalidRotationIndex),
        }
    }
}

impl Into<usize> for Rotation {
    fn into(self) -> usize {
        match self {
            Self::Normal => 0,
            Self::Prime => 1,
            Self::Two => 2,
        }
    }
}
