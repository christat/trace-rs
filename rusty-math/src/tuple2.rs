use std::fmt;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tuple2([f32; 2]);

impl Tuple2 {
  pub fn new(x: f32, y: f32) -> Self {
    Self([x, y])
  }

  pub fn x(&self) -> f32 {
    self.0[0]
  }

  pub fn y(&self) -> f32 {
    self.0[1]
  }

  pub fn length_squared(&self) -> f32 {
    self.x().powi(2) +
    self.y().powi(2)
  }

  pub fn length(&self) -> f32 {
    self.length_squared().sqrt()
  }

  pub fn normalize(&mut self) {
    let inverse_length = 1.0 / self.length();
    self.0[0] *= inverse_length;
    self.0[1] *= inverse_length;
  }

  pub fn normalized(&self) -> Self {
    let inverse_length = 1.0 / self.length();
    Self([
      self.x() * inverse_length,
      self.y() * inverse_length
    ])
  }

  pub fn dot(lhs: Self, rhs: Self) -> f32 {
    lhs.x() * rhs.x() + lhs.y() * rhs.y()
  }
}

impl fmt::Display for Tuple2 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[x: {}, y: {}]", self.x(), self.y())
  }
}

impl Add for Tuple2 {
  type Output = Self;

  fn add(self, other: Tuple2) -> Self::Output {
    Tuple2([
      self.x() + other.x(),
      self.y() + other.y()
    ])
  }
}

impl Sub for Tuple2 {
  type Output = Self;

  fn sub(self, other: Tuple2) -> Self::Output {
    Tuple2([
      self.x() - other.x(),
      self.y() - other.y()
    ])
  }
}

impl Neg for Tuple2 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Tuple2([
      -self.x(),
      -self.y()
    ])
  }
}

impl Mul<f32> for Tuple2 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    Tuple2([
      self.x() * rhs,
      self.y() * rhs
    ])
  }
}

impl Mul<Tuple2> for f32 {
  type Output = Tuple2;

  fn mul(self, rhs: Tuple2) -> Self::Output {
    Tuple2([
      rhs.x() * self,
      rhs.y() * self
    ])
  }
}

impl MulAssign<f32> for Tuple2 {
  fn mul_assign(&mut self, rhs: f32) {
    self.0[0] *= rhs;
    self.0[1] *= rhs;
  }
}

impl Div<f32> for Tuple2 {
  type Output = Self;

  fn div(self, rhs: f32) -> Self::Output {
    let inverse_rhs = 1.0 / rhs;
    Tuple2([
      self.x() * inverse_rhs,
      self.y() * inverse_rhs
    ])
  }
}

impl DivAssign<f32> for Tuple2 {
  fn div_assign(&mut self, rhs: f32) {
    let inverse_rhs = 1.0 / rhs;
    self.0[0] *= inverse_rhs;
    self.0[1] *= inverse_rhs;
  }
}

#[cfg(test)]
mod tests {
  use super::Tuple2;
  use crate::test_utils::cmp_f32;

  #[test]
  fn implements_constructor() {
    assert_eq!(Tuple2::new(4.3, -4.2), Tuple2([4.3, -4.2]));
  }

  #[test]
  fn implements_accessors() {
      let a1 = Tuple2::new(4.3, -4.2);
      assert_eq!(4.3, a1.x());
      assert_eq!(-4.2, a1.y());
  }

  #[test]
  fn implements_add() {
      let a1 = Tuple2::new(3.0, -2.0);
      let a2 = Tuple2::new(-2.0, 3.0);
      assert_eq!(Tuple2::new(1.0, 1.0), a1 + a2);
  }

  #[test]
  fn implements_sub() {
      let p1 = Tuple2::new(3.0, 2.0);
      let p2 = Tuple2::new(5.0, 6.0);
      assert_eq!(Tuple2::new(-2.0, -4.0), p1 - p2);

      let p = Tuple2::new(3.0, 2.0);
      let v = Tuple2::new(5.0, 6.0);
      assert_eq!(Tuple2::new(-2.0, -4.0), p - v);

      let v1 = Tuple2::new(3.0, 2.0);
      let v2 = Tuple2::new(5.0, 6.0);
      assert_eq!(Tuple2::new(-2.0, -4.0), v1 - v2);
  }

  #[test]
  fn implements_neg() {
    let a = Tuple2::new(1.0, -2.0);
    assert_eq!(Tuple2::new(-1.0, 2.0), -a);
  }

  #[test]
  fn implements_tuple_mul_f32() {
    let a = Tuple2::new(1.0, -2.0);
    assert_eq!(Tuple2::new(3.5, -7.0), a * 3.5);
    assert_eq!(Tuple2::new(0.5, -1.0), a * 0.5);
  }

  #[test]
  fn implements_f32_mul_tuple() {
    let a = Tuple2::new(1.0, -2.0);
    assert_eq!(Tuple2::new(3.5, -7.0), 3.5 * a);
    assert_eq!(Tuple2::new(0.5, -1.0), 0.5 * a);
  }

  #[test]
  fn implements_mulassign_f32() {
    let mut a = Tuple2::new(1.0, -2.0);
    a *= 3.5;
    assert_eq!(Tuple2::new(3.5, -7.0), a);
  }

  #[test]
  fn implements_tuple_div_f32() {
    let a = Tuple2::new(1.0, -2.0);
    assert_eq!(Tuple2::new(0.5, -1.0), a / 2.0);
  }

   #[test]
  fn implements_divassign_f32() {
    let mut a = Tuple2::new(1.0, -2.0);
    a /= 4.0;
    assert_eq!(Tuple2::new(0.25, -0.5), a);
  }

  #[test]
  fn implements_length_squared() {
    let vx = Tuple2::new(1.0, 0.0);
    assert_eq!(1.0, vx.length_squared());

    let vy = Tuple2::new(0.0, 1.0);
    assert_eq!(1.0, vy.length_squared());

    let v0 = Tuple2::new(1.0, 2.0);
    assert_eq!(5.0, v0.length_squared());

    let v1 = Tuple2::new(-1.0, -2.0);
    assert_eq!(5.0, v1.length_squared());
  }

  #[test]
  fn implements_length() {
    let vx = Tuple2::new(1.0, 0.0);
    assert_eq!(1.0, vx.length());

    let vy = Tuple2::new(0.0, 1.0);
    assert_eq!(1.0, vy.length());

    let v0 = Tuple2::new(1.0, 2.0);
    assert_eq!(f32::sqrt(5.0), v0.length());

    let v1 = Tuple2::new(-1.0, -2.0);
    assert_eq!(f32::sqrt(5.0), v1.length());
  }

  #[test]
  fn implements_normalize() {
    let mut v0 = Tuple2::new(4.0, 0.0);
    v0.normalize();
    assert_eq!(Tuple2::new(1.0, 0.0), v0);

    let mut v1 = Tuple2::new(1.0, 2.0);
    v1.normalize();
    let sqrt5 = f32::sqrt(5.0);
    assert_eq!(Tuple2::new(1.0 / sqrt5, 2.0 / sqrt5), v1);
    assert!(cmp_f32(1.0, v1.length()));
  }

  #[test]
  fn implements_normalized() {
    let v0 = Tuple2::new(4.0, 0.0).normalized();
    assert_eq!(Tuple2::new(1.0, 0.0), v0);

    let v1 = Tuple2::new(1.0, 2.0).normalized();
    let sqrt5 = f32::sqrt(5.0);
    assert_eq!(Tuple2::new(1.0 / sqrt5, 2.0 / sqrt5), v1);
    assert!(cmp_f32(1.0, v1.length()));
  }
}