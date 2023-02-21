use std::str::FromStr;

use crate::traits::Indexable;

use super::{Face, ParseFaceError, Rotation};

pub struct Action {
    face: Face,
    rotation: Rotation,
}

impl Indexable<usize, Action> for Action {
    fn from_index(index: usize) -> Option<Action> {
        Some(Action {
            face: Face::from_index(index / 3)?,
            rotation: Rotation::from_index(index % 3)?,
        })
    }

    fn to_index(&self) -> usize {
        self.face.to_index() * 3 + self.rotation.to_index()
    }
}

pub enum ParseActionError {
    StringSpliceError,
    ParseFaceError,
    ParseRotationError,
}

impl From<ParseFaceError> for ParseActionError {
    fn from(_: ParseFaceError) -> Self {
        Self::ParseFaceError
    }
}

impl FromStr for Action {
    type Err = ParseActionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let face_str = s.get(0..1).ok_or(ParseActionError::StringSpliceError)?;
        let face = Face::from_str(face_str)?;

        let rotation = match s.get(1..2) {
            None => Ok(Rotation::Normal),
            Some(rotation_str) => match rotation_str {
                "P" => Ok(Rotation::Prime),
                "2" => Ok(Rotation::Two),
                _ => Err(ParseActionError::ParseRotationError),
            },
        }?;

        Ok(Self { face, rotation })
    }
}
