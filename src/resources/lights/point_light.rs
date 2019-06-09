use crate::components::Color;
use yaac::Tuple4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointLight {
  pub position: Tuple4,
  pub intensity: Color
}

impl PointLight {
  pub fn new(position: Tuple4, intensity: Color) -> Self {
    Self {
      position: position,
      intensity: intensity
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Color, PointLight};
  use yaac::Tuple4;

  #[test]
  fn constructor() {
    let position = Tuple4::point(0.0, 0.0, 0.0);
    let color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(position, color);
    assert_eq!(PointLight { position: position, intensity: color }, light);
  }

}