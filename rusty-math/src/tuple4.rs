use std::fmt;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use crate::Tuple3;
use crate::errors::PointCrossProductError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tuple4([f32; 4]);

impl Tuple4 {
  pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
    Self([x, y, z, w])
  }

  pub fn point(x: f32, y: f32, z: f32) -> Self {
    Self([x, y, z, 1.0])
  }

  pub fn vector(x: f32, y: f32, z: f32) -> Self {
    Self([x, y, z, 0.0])
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

  pub fn w(&self) -> f32 {
    self.0[3]
  }

  pub fn xyz(&self) -> Tuple3 {
    Tuple3::new(self.x(), self.y(), self.z())
  }

  pub fn xyw(&self) -> Tuple3 {
    Tuple3::new(self.x(), self.y(), self.w())
  }

  pub fn xzw(&self) -> Tuple3 {
    Tuple3::new(self.x(), self.z(), self.w())
  }

  pub fn yzw(&self) -> Tuple3 {
    Tuple3::new(self.y(), self.z(), self.w())
  }

  pub fn is_vector(&self) -> bool {
    if self.w() == 0.0 { true } else { false }
  }

  pub fn length_squared(&self) -> f32 {
    self.x().powi(2) +
    self.y().powi(2) +
    self.z().powi(2) +
    self.w().powi(2)
  }

  pub fn length(&self) -> f32 {
    self.length_squared().sqrt()
  }

  pub fn normalize(&mut self) {
    let inverse_length = 1.0 / self.length();
    self.0[0] *= inverse_length;
    self.0[1] *= inverse_length;
    self.0[2] *= inverse_length;
    self.0[3] *= inverse_length;
  }

  pub fn normalized(&self) -> Self {
    let inverse_length = 1.0 / self.length();
    Self([
      self.x() * inverse_length,
      self.y() * inverse_length,
      self.z() * inverse_length,
      self.w() * inverse_length
    ])
  }

  pub fn dot(lhs: Self, rhs: Self) -> f32 {
    lhs.x() * rhs.x() +
    lhs.y() * rhs.y() +
    lhs.z() * rhs.z() +
    lhs.w() * rhs.w()
  }

  pub fn cross(lhs: Self, rhs: Self) -> Result<Self, PointCrossProductError> {
    if !lhs.is_vector() || !rhs.is_vector() {
      Err(PointCrossProductError)
    } else {
      Ok(
        Tuple4([
          lhs.y() * rhs.z() - lhs.z() * rhs.y(),
          lhs.z() * rhs.x() - lhs.x() * rhs.z(),
          lhs.x() * rhs.y() - lhs.y() * rhs.x(),
          0.0
        ])
      )
    }
  }
}

impl fmt::Display for Tuple4 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[x: {}, y: {}, z: {}, w: {}]", self.x(), self.y(), self.z(), self.w())
  }
}

impl Add for Tuple4 {
  type Output = Self;

  fn add(self, other: Tuple4) -> Self::Output {
    Tuple4([
      self.x() + other.x(),
      self.y() + other.y(),
      self.z() + other.z(),
      self.w() + other.w()
    ])
  }
}

impl Sub for Tuple4 {
  type Output = Self;

  fn sub(self, other: Tuple4) -> Self::Output {
    Tuple4([
      self.x() - other.x(),
      self.y() - other.y(),
      self.z() - other.z(),
      self.w() - other.w()
    ])
  }
}

impl Neg for Tuple4 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Tuple4([
      -self.x(),
      -self.y(),
      -self.z(),
      -self.w()
    ])
  }
}

impl Mul<f32> for Tuple4 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    Tuple4([
      self.x() * rhs,
      self.y() * rhs,
      self.z() * rhs,
      self.w() * rhs
    ])
  }
}

impl Mul<Tuple4> for f32 {
  type Output = Tuple4;

  fn mul(self, rhs: Tuple4) -> Self::Output {
    Tuple4([
      rhs.x() * self,
      rhs.y() * self,
      rhs.z() * self,
      rhs.w() * self
    ])
  }
}

impl MulAssign<f32> for Tuple4 {
  fn mul_assign(&mut self, rhs: f32) {
    self.0[0] *= rhs;
    self.0[1] *= rhs;
    self.0[2] *= rhs;
    self.0[3] *= rhs;
  }
}

impl Div<f32> for Tuple4 {
  type Output = Self;

  fn div(self, rhs: f32) -> Self::Output {
    let inverse_rhs = 1.0 / rhs;
    Tuple4([
      self.x() * inverse_rhs,
      self.y() * inverse_rhs,
      self.z() * inverse_rhs,
      self.w() * inverse_rhs
    ])
  }
}

impl DivAssign<f32> for Tuple4 {
  fn div_assign(&mut self, rhs: f32) {
    let inverse_rhs = 1.0 / rhs;
    self.0[0] *= inverse_rhs;
    self.0[1] *= inverse_rhs;
    self.0[2] *= inverse_rhs;
    self.0[3] *= inverse_rhs;
  }
}

#[cfg(test)]
mod tests {
  use super::{PointCrossProductError, Tuple3, Tuple4};
  use crate::test_utils::{cmp_f32};

  #[test]
  fn implements_constructors() {
    assert_eq!(Tuple4([4.3, -4.2, 3.1, 5.0]), Tuple4::new(4.3, -4.2, 3.1, 5.0));
    assert_eq!(Tuple4([4.3, -4.2, 3.1, 1.0]), Tuple4::point(4.3, -4.2, 3.1));
    assert_eq!(Tuple4([4.3, -4.2, 3.1, 0.0]), Tuple4::vector(4.3, -4.2, 3.1));
  }

  #[test]
  fn implements_accessors() {
      let vector = Tuple4::vector(4.3, -4.2, 3.1);
      assert_eq!(4.3, vector.x());
      assert_eq!(-4.2, vector.y());
      assert_eq!(3.1, vector.z());
      assert_eq!(0.0, vector.w());
      assert_eq!(true, vector.is_vector());

      let point = Tuple4::point(4.3, -4.2, 3.1);
      assert_eq!(4.3, point.x());
      assert_eq!(-4.2, point.y());
      assert_eq!(3.1, point.z());
      assert_eq!(1.0, point.w());
      assert_eq!(false, point.is_vector());
  }

  #[test]
  fn implements_swizzled_sub() {
    let vector = Tuple4::vector(4.3, -4.2, 3.1);
    assert_eq!(Tuple3::new(4.3, -4.2, 3.1), vector.xyz());
    assert_eq!(Tuple3::new(4.3, -4.2, 0.0), vector.xyw());
    assert_eq!(Tuple3::new(4.3, 3.1, 0.0), vector.xzw());
    assert_eq!(Tuple3::new(-4.2, 3.1, 0.0), vector.yzw());
  }

  #[test]
  fn implements_add() {
      let a1 = Tuple4::new(3.0, -2.0, 5.0, 1.0);
      let a2 = Tuple4::new(-2.0, 3.0, 1.0, 0.0);
      assert_eq!(Tuple4::new(1.0, 1.0, 6.0, 1.0), a1 + a2);
  }

  #[test]
  fn implements_sub() {
      let p1 = Tuple4::point(3.0, 2.0, 1.0);
      let p2 = Tuple4::point(5.0, 6.0, 7.0);
      assert_eq!(Tuple4::vector(-2.0, -4.0, -6.0), p1 - p2);

      let p = Tuple4::point(3.0, 2.0, 1.0);
      let v = Tuple4::vector(5.0, 6.0, 7.0);
      assert_eq!(Tuple4::point(-2.0, -4.0, -6.0), p - v);

      let v1 = Tuple4::vector(3.0, 2.0, 1.0);
      let v2 = Tuple4::vector(5.0, 6.0, 7.0);
      assert_eq!(Tuple4::vector(-2.0, -4.0, -6.0), v1 - v2);
  }

  #[test]
  fn implements_neg() {
    let a = Tuple4::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(Tuple4::new(-1.0, 2.0, -3.0, 4.0), -a);
  }

  #[test]
  fn implements_tuple_mul_f32() {
    let a = Tuple4::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(Tuple4::new(3.5, -7.0, 10.5, -14.0), a * 3.5);
    assert_eq!(Tuple4::new(0.5, -1.0, 1.5, -2.0), a * 0.5);
  }

  #[test]
  fn implements_f32_mul_tuple() {
    let a = Tuple4::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(Tuple4::new(3.5, -7.0, 10.5, -14.0), 3.5 * a);
    assert_eq!(Tuple4::new(0.5, -1.0, 1.5, -2.0), 0.5 * a);
  }

  #[test]
  fn implements_mulassign_f32() {
    let mut a = Tuple4::new(1.0, -2.0, 3.0, -4.0);
    a *= 3.5;
    assert_eq!(Tuple4::new(3.5, -7.0, 10.5, -14.0), a);
  }

  #[test]
  fn implements_tuple_div_f32() {
    let a = Tuple4::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(Tuple4::new(0.5, -1.0, 1.5, -2.0), a / 2.0);
  }

  #[test]
  fn implements_divassign_f32() {
    let mut a = Tuple4::new(1.0, -2.0, 4.0, -8.0);
    a /= 4.0;
    assert_eq!(Tuple4::new(0.25, -0.5, 1.0, -2.0), a);
  }

  #[test]
  fn implements_length_squared() {
    let vx = Tuple4::vector(1.0, 0.0, 0.0);
    assert_eq!(1.0, vx.length_squared());

    let vy = Tuple4::vector(0.0, 1.0, 0.0);
    assert_eq!(1.0, vy.length_squared());

    let vz = Tuple4::vector(0.0, 0.0, 1.0);
    assert_eq!(1.0, vz.length_squared());

    let v0 = Tuple4::vector(1.0, 2.0, 3.0);
    assert_eq!(14.0, v0.length_squared());

    let v1 = Tuple4::vector(-1.0, -2.0, -3.0);
    assert_eq!(14.0, v1.length_squared());
  }

  #[test]
  fn implements_length() {
    let vx = Tuple4::vector(1.0, 0.0, 0.0);
    assert_eq!(1.0, vx.length());

    let vy = Tuple4::vector(0.0, 1.0, 0.0);
    assert_eq!(1.0, vy.length());

    let vz = Tuple4::vector(0.0, 0.0, 1.0);
    assert_eq!(1.0, vz.length());

    let v0 = Tuple4::vector(1.0, 2.0, 3.0);
    assert_eq!(f32::sqrt(14.0), v0.length());

    let v1 = Tuple4::vector(-1.0, -2.0, -3.0);
    assert_eq!(f32::sqrt(14.0), v1.length());
  }

  #[test]
  fn implements_normalize() {
    let mut v0 = Tuple4::vector(4.0, 0.0, 0.0);
    v0.normalize();
    assert_eq!(Tuple4::vector(1.0, 0.0, 0.0), v0);

    let mut v1 = Tuple4::vector(1.0, 2.0, 3.0);
    v1.normalize();
    let sqrt14 = f32::sqrt(14.0);
    assert_eq!(Tuple4::vector(1.0 / sqrt14, 2.0 / sqrt14, 3.0 / sqrt14), v1);
    assert!(cmp_f32(1.0, v1.length()));
  }

  #[test]
  fn implements_normalized() {
    let v0 = Tuple4::vector(4.0, 0.0, 0.0).normalized();
    assert_eq!(Tuple4::vector(1.0, 0.0, 0.0), v0);

    let v1 = Tuple4::vector(1.0, 2.0, 3.0).normalized();
    let sqrt14 = f32::sqrt(14.0);
    assert_eq!(Tuple4::vector(1.0 / sqrt14, 2.0 / sqrt14, 3.0 / sqrt14), v1);
    assert!(cmp_f32(1.0, v1.length()));
  }

  #[test]
  fn implements_dot_product() {
    let a = Tuple4::vector(1.0, 2.0, 3.0);
    let b = Tuple4::vector(2.0, 3.0, 4.0);
    assert_eq!(20.0, Tuple4::dot(a, b));
  }

  #[test]
  fn implements_cross_product() {
    let a = Tuple4::vector(1.0, 2.0, 3.0);
    let b = Tuple4::vector(2.0, 3.0, 4.0);
    assert_eq!(Ok(Tuple4::vector(-1.0, 2.0, -1.0)), Tuple4::cross(a, b));
    assert_eq!(Ok(Tuple4::vector(1.0, -2.0, 1.0)), Tuple4::cross(b, a));

    let c = Tuple4::point(2.0, 3.0, 4.0);
    assert_eq!(Err(PointCrossProductError), Tuple4::cross(a, c));
    assert_eq!(Err(PointCrossProductError), Tuple4::cross(c, a));
    assert_eq!(Err(PointCrossProductError), Tuple4::cross(c, c));
  }
}