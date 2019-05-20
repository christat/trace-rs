extern crate rusty_math as rm;

use rm::{Matrix4, Tuple4};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
  pub origin: Tuple4,
  pub direction: Tuple4
}

impl Ray {
  pub fn new(origin: Tuple4, direction: Tuple4) -> Self {
    Self {
      origin: origin,
      direction: direction
    }
  }

  pub fn point_at(&self, t: f32) -> Tuple4 {
    self.origin + self.direction * t
  }

  pub fn transform(&self, t: Matrix4) -> Self {
    Self {
      origin: t * self.origin,
      direction: t * self.direction
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Ray, Matrix4, Tuple4};

  #[test]
  fn implements_constructor() {
    let origin = Tuple4::point(1.0, 2.0, 3.0);
    let direction = Tuple4::vector(4.0, 5.0, 6.0);
    let r = Ray::new(origin, direction);
    assert_eq!(origin, r.origin);
    assert_eq!(direction, r.direction);
  }

  #[test]
  fn implements_point_at() {
    let r = Ray::new(Tuple4::point(2.0, 3.0, 4.0), Tuple4::vector(1.0, 0.0, 0.0));
    assert_eq!(Tuple4::point(2.0, 3.0, 4.0), r.point_at(0.0));
    assert_eq!(Tuple4::point(3.0, 3.0, 4.0), r.point_at(1.0));
    assert_eq!(Tuple4::point(1.0, 3.0, 4.0), r.point_at(-1.0));
    assert_eq!(Tuple4::point(4.5, 3.0, 4.0), r.point_at(2.5));
  }

  #[test]
  fn implements_transform() {
    let r = Ray::new(Tuple4::point(1.0, 2.0, 3.0), Tuple4::vector(0.0, 1.0, 0.0));
    let m = Matrix4::translation(3.0, 4.0, 5.0);
    let r2 = r.transform(m);
    assert_eq!(Tuple4::point(4.0, 6.0, 8.0), r2.origin);
    assert_eq!(Tuple4::vector(0.0, 1.0, 0.0), r2.direction);

    let m = Matrix4::scaling(2.0, 3.0, 4.0);
    let r2 = r.transform(m);
    assert_eq!(Tuple4::point(2.0, 6.0, 12.0), r2.origin);
    assert_eq!(Tuple4::vector(0.0, 3.0, 0.0), r2.direction);
  }
}