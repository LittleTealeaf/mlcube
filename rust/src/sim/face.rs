use std::str::FromStr;

use crate::traits::Indexable;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Face {
    R,
    U,
    F,
    L,
    D,
    B,
}

impl Indexable<usize, Self> for Face {
    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::U),
            1 => Some(Self::L),
            2 => Some(Self::F),
            3 => Some(Self::R),
            4 => Some(Self::B),
            5 => Some(Self::D),
            _ => None,
        }
    }

    fn to_index(&self) -> usize {
        match self {
            Self::U => 0,
            Self::L => 1,
            Self::F => 2,
            Self::R => 3,
            Self::B => 4,
            Self::D => 5,
        }
    }
}


pub struct ParseFaceError;

impl FromStr for Face {
    type Err = ParseFaceError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::U),
            "L" => Ok(Self::L),
            "F" => Ok(Self::F),
            "R" => Ok(Self::R),
            "B" => Ok(Self::B),
            "D" => Ok(Self::D),
            _ => Err(ParseFaceError),
        }
    }
}

pub struct InvalidFaceIndex;

impl TryFrom<usize> for Face {
    type Error = InvalidFaceIndex;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::U),
            1 => Ok(Self::L),
            2 => Ok(Self::F),
            3 => Ok(Self::R),
            4 => Ok(Self::B),
            5 => Ok(Self::D),
            _ => Err(InvalidFaceIndex),
        }
    }
}

impl Into<usize> for Face {
    fn into(self) -> usize {
        match self {
            Self::U => 0,
            Self::L => 1,
            Self::F => 2,
            Self::R => 3,
            Self::B => 4,
            Self::D => 5,
        }
    }
}
