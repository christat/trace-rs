use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color([f32; 3]);

impl Color {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self([x, y, z])
  }

  pub fn r(&self) -> f32 {
    self.0[0]
  }

  pub fn g(&self) -> f32 {
    self.0[1]
  }

  pub fn b(&self) -> f32 {
    self.0[2]
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "rgb({}, {}, {})", self.r(), self.g(), self.b())
  }
}

impl Add for Color {
  type Output = Self;

  fn add(self, other: Color) -> Self::Output {
    Color([
      self.r() + other.r(),
      self.g() + other.g(),
      self.b() + other.b()
    ])
  }
}

impl Sub for Color {
  type Output = Self;

  fn sub(self, other: Color) -> Self::Output {
    Color([
      self.r() - other.r(),
      self.g() - other.g(),
      self.b() - other.b()
    ])
  }
}

/// Hadamard/Schur product of colors
impl Mul<Color> for Color {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Color([
      self.r() * rhs.r(),
      self.g() * rhs.g(),
      self.b() * rhs.b()
    ])
  }
}

impl Mul<f32> for Color {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    Color([
      self.r() * rhs,
      self.g() * rhs,
      self.b() * rhs
    ])
  }
}

impl Mul<Color> for f32 {
  type Output = Color;

  fn mul(self, rhs: Color) -> Self::Output {
    Color([
      rhs.r() * self,
      rhs.g() * self,
      rhs.b() * self
    ])
  }
}

#[cfg(test)]
mod tests {
  use super::Color;

  static EPSILON: f32 = 0.0001;

  fn f32_cmp(a: f32, b: f32) -> bool {
    if (a - b) < EPSILON { true } else { false }
  }

  fn equal(c1: Color, c2: Color) -> bool {
    if f32_cmp(c1.r(), c2.r()) &&
       f32_cmp(c1.g(), c2.g()) &&
       f32_cmp(c1.b(), c2.b())
    { true } else { false }
  }

  #[test]
  fn implements_constructor() {
    assert_eq!(Color::new(-0.5, 0.4, 1.7), Color([-0.5, 0.4, 1.7]));
  }

  #[test]
  fn implements_accessors() {
    let c = Color::new(-0.5, 0.4, 1.7);
    assert_eq!(c.r(), -0.5);
    assert_eq!(c.g(), 0.4);
    assert_eq!(c.b(), 1.7);
  }

  #[test]
  fn implements_add() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    let res = Color::new(1.6, 0.7, 1.0);
    assert!(equal(res, c1 + c2), "{}, {}", res, c1 + c2);
  }

  #[test]
  fn implements_sub() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    assert!(equal(Color::new(0.2, 0.5, 0.5), c1 - c2));
  }

  #[test]
  fn implements_color_mul_f32() {
    let c = Color::new(0.2, 0.3, 0.4);
    assert!(equal(Color::new(0.4, 0.6, 0.8), c * 2.0));
  }

  #[test]
  fn implements_f32_mul_color() {
    let c = Color::new(0.2, 0.3, 0.4);
    assert!(equal(Color::new(0.4, 0.6, 0.8), 2.0 * c));
  }

  #[test]
  fn implements_product() {
    let c1 = Color::new(1.0, 0.2, 0.4);
    let c2 = Color::new(0.9, 1.0, 0.1);
    assert!(equal(Color::new(0.9, 0.2, 0.04), c1 * c2));
  }
}