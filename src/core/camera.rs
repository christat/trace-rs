use rm::Tuple4;

pub struct Camera {
  position: Tuple4
}

impl Camera {
  pub fn new(position: Tuple4) -> Self {
    Self {
      position: position
    }
  }
}