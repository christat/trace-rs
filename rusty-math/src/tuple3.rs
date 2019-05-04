use std::fmt;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use crate::Tuple2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tuple3([f32; 3]);

impl Tuple3 {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self([x, y, z])
  }

  pub fn x(&self) -> f32 {
    self.0[0]
  }

  pub fn y(&self) -> f32 {
    self.0[1]
  }

  pub fn z(&self) -> f32 {
    self.0[2]
  }

  pub fn xy(&self) -> Tuple2 {
    Tuple2::new(self.0[0], self.0[1])
  }

  pub fn xz(&self) -> Tuple2 {
    Tuple2::new(self.0[0], self.0[2])
  }

  pub fn yz(&self) -> Tuple2 {
    Tuple2::new(self.0[1], self.0[2])
  }

  pub fn length_squared(&self) -> f32 {
    self.x().powi(2) +
    self.y().powi(2) +
    self.z().powi(2)
  }

  pub fn length(&self) -> f32 {
    self.length_squared().sqrt()
  }

  pub fn normalize(&mut self) {
    let inverse_length = 1.0 / self.length();
    self.0[0] *= inverse_length;
    self.0[1] *= inverse_length;
    self.0[2] *= inverse_length;
  }

  pub fn normalized(&self) -> Self {
    let inverse_length = 1.0 / self.length();
    Self([
      self.x() * inverse_length,
      self.y() * inverse_length,
      self.z() * inverse_length
    ])
  }

  pub fn dot(lhs: Self, rhs: Self) -> f32 {
    lhs.x() * rhs.x() +
    lhs.y() * rhs.y() +
    lhs.z() * rhs.z()
  }

  pub fn cross(lhs: Self, rhs: Self) -> Self {
    Tuple3([
      lhs.y() * rhs.z() - lhs.z() * rhs.y(),
      lhs.z() * rhs.x() - lhs.x() * rhs.z(),
      lhs.x() * rhs.y() - lhs.y() * rhs.x()
    ])
  }
}

impl fmt::Display for Tuple3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[x: {}, y: {}, z: {}]", self.x(), self.y(), self.z())
  }
}

impl Add for Tuple3 {
  type Output = Self;

  fn add(self, other: Tuple3) -> Self::Output {
    Tuple3([
      self.x() + other.x(),
      self.y() + other.y(),
      self.z() + other.z()
    ])
  }
}

impl Sub for Tuple3 {
  type Output = Self;

  fn sub(self, other: Tuple3) -> Self::Output {
    Tuple3([
      self.x() - other.x(),
      self.y() - other.y(),
      self.z() - other.z()
    ])
  }
}

impl Neg for Tuple3 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Tuple3([
      -self.x(),
      -self.y(),
      -self.z()
    ])
  }
}

impl Mul<f32> for Tuple3 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    Tuple3([
      self.x() * rhs,
      self.y() * rhs,
      self.z() * rhs
    ])
  }
}

impl Mul<Tuple3> for f32 {
  type Output = Tuple3;

  fn mul(self, rhs: Tuple3) -> Self::Output {
    Tuple3([
      rhs.x() * self,
      rhs.y() * self,
      rhs.z() * self
    ])
  }
}

impl MulAssign<f32> for Tuple3 {
  fn mul_assign(&mut self, rhs: f32) {
    self.0[0] *= rhs;
    self.0[1] *= rhs;
    self.0[2] *= rhs;
  }
}

impl Div<f32> for Tuple3 {
  type Output = Self;

  fn div(self, rhs: f32) -> Self::Output {
    let inverse_rhs = 1.0 / rhs;
    Tuple3([
      self.x() * inverse_rhs,
      self.y() * inverse_rhs,
      self.z() * inverse_rhs
    ])
  }
}

impl DivAssign<f32> for Tuple3 {
  fn div_assign(&mut self, rhs: f32) {
    let inverse_rhs = 1.0 / rhs;
    self.0[0] *= inverse_rhs;
    self.0[1] *= inverse_rhs;
    self.0[2] *= inverse_rhs;
  }
}

#[cfg(test)]
mod tests {
  use super::{Tuple2, Tuple3};
  use crate::test_utils::cmp_f32;

  #[test]
  fn implements_constructor() {
    assert_eq!(Tuple3([4.3, -4.2, 3.1]), Tuple3::new(4.3, -4.2, 3.1));
  }

  #[test]
  fn implements_accessors() {
      let a1 = Tuple3::new(4.3, -4.2, 3.1);
      assert_eq!(4.3, a1.x());
      assert_eq!(-4.2, a1.y());
      assert_eq!(3.1, a1.z());
  }

  #[test]
  fn implements_swizzled_sub() {
    let a1 = Tuple3::new(4.3, -4.2, 3.1);
    assert_eq!(Tuple2::new(4.3, -4.2), a1.xy());
    assert_eq!(Tuple2::new(4.3, 3.1), a1.xz());
    assert_eq!(Tuple2::new(-4.2, 3.1), a1.yz());
  }

  #[test]
  fn implements_add() {
      let a1 = Tuple3::new(3.0, -2.0, 5.0);
      let a2 = Tuple3::new(-2.0, 3.0, 1.0);
      assert_eq!(Tuple3::new(1.0, 1.0, 6.0), a1 + a2);
  }

  #[test]
  fn implements_sub() {
      let p1 = Tuple3::new(3.0, 2.0, 1.0);
      let p2 = Tuple3::new(5.0, 6.0, 7.0);
      assert_eq!(Tuple3::new(-2.0, -4.0, -6.0), p1 - p2);

      let p = Tuple3::new(3.0, 2.0, 1.0);
      let v = Tuple3::new(5.0, 6.0, 7.0);
      assert_eq!(Tuple3::new(-2.0, -4.0, -6.0), p - v);

      let v1 = Tuple3::new(3.0, 2.0, 1.0);
      let v2 = Tuple3::new(5.0, 6.0, 7.0);
      assert_eq!(Tuple3::new(-2.0, -4.0, -6.0), v1 - v2);
  }

  #[test]
  fn implements_neg() {
    let a = Tuple3::new(1.0, -2.0, 3.0);
    assert_eq!(Tuple3::new(-1.0, 2.0, -3.0), -a);
  }

  #[test]
  fn implements_tuple_mul_f32() {
    let a = Tuple3::new(1.0, -2.0, 3.0);
    assert_eq!(Tuple3::new(3.5, -7.0, 10.5), a * 3.5);
    assert_eq!(Tuple3::new(0.5, -1.0, 1.5), a * 0.5);
  }

  #[test]
  fn implements_f32_mul_tuple() {
    let a = Tuple3::new(1.0, -2.0, 3.0);
    assert_eq!(Tuple3::new(3.5, -7.0, 10.5), 3.5 * a);
    assert_eq!(Tuple3::new(0.5, -1.0, 1.5), 0.5 * a);
  }

  #[test]
  fn implements_mulassign_f32() {
    let mut a = Tuple3::new(1.0, -2.0, 3.0);
    a *= 3.5;
    assert_eq!(Tuple3::new(3.5, -7.0, 10.5), a);
  }

  #[test]
  fn implements_tuple_div_f32() {
    let a = Tuple3::new(1.0, -2.0, 3.0);
    assert_eq!(Tuple3::new(0.5, -1.0, 1.5), a / 2.0);
  }

   #[test]
  fn implements_divassign_f32() {
    let mut a = Tuple3::new(1.0, -2.0, 4.0);
    a /= 4.0;
    assert_eq!(Tuple3::new(0.25, -0.5, 1.0), a);
  }

  #[test]
  fn implements_length_squared() {
    let vx = Tuple3::new(1.0, 0.0, 0.0);
    assert_eq!(1.0, vx.length_squared());

    let vy = Tuple3::new(0.0, 1.0, 0.0);
    assert_eq!(1.0, vy.length_squared());

    let vz = Tuple3::new(0.0, 0.0, 1.0);
    assert_eq!(1.0, vz.length_squared());

    let v0 = Tuple3::new(1.0, 2.0, 3.0);
    assert_eq!(14.0, v0.length_squared());

    let v1 = Tuple3::new(-1.0, -2.0, -3.0);
    assert_eq!(14.0, v1.length_squared());
  }

  #[test]
  fn implements_length() {
    let vx = Tuple3::new(1.0, 0.0, 0.0);
    assert_eq!(1.0, vx.length());

    let vy = Tuple3::new(0.0, 1.0, 0.0);
    assert_eq!(1.0, vy.length());

    let vz = Tuple3::new(0.0, 0.0, 1.0);
    assert_eq!(1.0, vz.length());

    let v0 = Tuple3::new(1.0, 2.0, 3.0);
    assert_eq!(f32::sqrt(14.0), v0.length());

    let v1 = Tuple3::new(-1.0, -2.0, -3.0);
    assert_eq!(f32::sqrt(14.0), v1.length());
  }

  #[test]
  fn implements_normalize() {
    let mut v0 = Tuple3::new(4.0, 0.0, 0.0);
    v0.normalize();
    assert_eq!(Tuple3::new(1.0, 0.0, 0.0), v0);

    let mut v1 = Tuple3::new(1.0, 2.0, 3.0);
    v1.normalize();
    let sqrt14 = f32::sqrt(14.0);
    assert_eq!(Tuple3::new(1.0 / sqrt14, 2.0 / sqrt14, 3.0 / sqrt14), v1);
    assert!(cmp_f32(1.0, v1.length()));
  }

  #[test]
  fn implements_normalized() {
    let v0 = Tuple3::new(4.0, 0.0, 0.0).normalized();
    assert_eq!(Tuple3::new(1.0, 0.0, 0.0), v0);

    let v1 = Tuple3::new(1.0, 2.0, 3.0).normalized();
    let sqrt14 = f32::sqrt(14.0);
    assert_eq!(Tuple3::new(1.0 / sqrt14, 2.0 / sqrt14, 3.0 / sqrt14), v1);
    assert!(cmp_f32(1.0, v1.length()));
  }

  #[test]
  fn implements_dot_product() {
    let a = Tuple3::new(1.0, 2.0, 3.0);
    let b = Tuple3::new(2.0, 3.0, 4.0);
    assert_eq!(20.0, Tuple3::dot(a, b));
  }

  #[test]
  fn implements_cross_product() {
    let a = Tuple3::new(1.0, 2.0, 3.0);
    let b = Tuple3::new(2.0, 3.0, 4.0);
    assert_eq!(Tuple3::new(-1.0, 2.0, -1.0), Tuple3::cross(a, b));
    assert_eq!(Tuple3::new(1.0, -2.0, 1.0), Tuple3::cross(b, a));
  }
}