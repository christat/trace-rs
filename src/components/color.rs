use std::fmt;
use std::ops::{Add, Mul, Sub};

pub const BLACK: Color = Color([0.0, 0.0, 0.0]);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color([f32; 3]);

impl Color {
  pub fn new(r: f32, g: f32, b: f32) -> Self {
    Self([r, g, b])
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

  pub fn to_channels_array(&self) -> [String; 3] {
    let r = f32::min(255.0, f32::max(0.0, (self.r() * 255.0).round())) as u8;
    let g = f32::min(255.0, f32::max(0.0, (self.g() * 255.0).round())) as u8;
    let b = f32::min(255.0, f32::max(0.0, (self.b() * 255.0).round())) as u8;
    [r.to_string(), g.to_string(), b.to_string()]
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

  mod methods {
    use super::super::Color;
    
    #[test]
    fn constructor() {
      assert_eq!(Color::new(-0.5, 0.4, 1.7), Color([-0.5, 0.4, 1.7]));
    }

    #[test]
    fn accessors() {
      let c = Color::new(-0.5, 0.4, 1.7);
      assert_eq!(c.r(), -0.5);
      assert_eq!(c.g(), 0.4);
      assert_eq!(c.b(), 1.7);
    }

    #[test]
    fn to_channels_array() {
      let c1 = Color::new(1.5, 0.0, 0.0);
      let c2 = Color::new(0.0, 0.5, 0.0);
      let c3 = Color::new(-0.5, 0.0, 1.0);
      assert_eq!("255 0 0".split_whitespace().collect::<Vec<_>>(), c1.to_channels_array());
      assert_eq!("0 128 0".split_whitespace().collect::<Vec<_>>(), c2.to_channels_array());
      assert_eq!("0 0 255".split_whitespace().collect::<Vec<_>>(), c3.to_channels_array());
    }
  }

  mod traits {
    use super::super::Color;
    use super::equal;

    #[test]
    fn add() {
      let c1 = Color::new(0.9, 0.6, 0.75);
      let c2 = Color::new(0.7, 0.1, 0.25);
      let res = Color::new(1.6, 0.7, 1.0);
      assert!(equal(res, c1 + c2), "{}, {}", res, c1 + c2);
    }

    #[test]
    fn sub() {
      let c1 = Color::new(0.9, 0.6, 0.75);
      let c2 = Color::new(0.7, 0.1, 0.25);
      assert!(equal(Color::new(0.2, 0.5, 0.5), c1 - c2));
    }

    #[test]
    fn mul_color() {
      let c1 = Color::new(1.0, 0.2, 0.4);
      let c2 = Color::new(0.9, 1.0, 0.1);
      assert!(equal(Color::new(0.9, 0.2, 0.04), c1 * c2));
    }

    #[test]
    fn mul_f32() {
      let c = Color::new(0.2, 0.3, 0.4);
      assert!(equal(Color::new(0.4, 0.6, 0.8), c * 2.0));
    }

    #[test]
    fn f32_mul_color() {
      let c = Color::new(0.2, 0.3, 0.4);
      assert!(equal(Color::new(0.4, 0.6, 0.8), 2.0 * c));
    }
  }
}