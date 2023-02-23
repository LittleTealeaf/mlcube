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
