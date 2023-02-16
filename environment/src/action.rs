

pub enum Face {
  R,U,F,L,D,B
}

impl Face {
  pub fn from_index(index: usize) -> Face {
    match index {
      0 => Self::R,
      1 => Self::U,
      2 => Self::F,
      3 => Self::L,
      4 => Self::D,
      _ => Self::B
    }
  }
}


pub enum Action {
  Normal(Face),
  Prime(Face),
  Two(Face)
}

impl Action {
  pub fn from_index(index: usize) -> Action {
    let face = Face::from_index(index / 3);
    match index % 3 {
      0 => Self::Normal(face),
      1 => Self::Prime(face),
      _ => Self::Two(face)
    }
  }
}
