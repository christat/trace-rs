mod point_light;

use crate::components::Color;
use rm::Tuple4;

pub use point_light::PointLight;

pub struct Lights(pub PointLight);

impl Default for Lights {
  fn default() -> Self {
    Self(PointLight::new(
      Tuple4::point(-10.0, 10.0, -10.0),
      Color::new(1.0, 1.0, 1.0),
    ))
  }
}
