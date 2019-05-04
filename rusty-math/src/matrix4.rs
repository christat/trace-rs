use std::ops::Mul;
use crate::Matrix3;
use crate::Tuple4;
use crate::errors::{MatrixInversionError, SubmatrixIndexError};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4 {
  pub c0: Tuple4,
  pub c1: Tuple4,
  pub c2: Tuple4,
  pub c3: Tuple4
}

impl Matrix4 {
  pub fn new(c0: Tuple4, c1: Tuple4, c2: Tuple4, c3: Tuple4) -> Self {
    Self { c0: c0, c1: c1, c2: c2, c3: c3 }
  }

  pub fn identity() -> Self {
    Self::new(
      Tuple4::new(1.0, 0.0, 0.0, 0.0),
      Tuple4::new(0.0, 1.0, 0.0, 0.0),
      Tuple4::new(0.0, 0.0, 1.0, 0.0),
      Tuple4::new(0.0, 0.0, 0.0, 1.0)
    )
  }

  pub fn r0(&self) -> Tuple4 {
    Tuple4::new(self.c0.x(), self.c1.x(), self.c2.x(), self.c3.x())
  }

  pub fn r1(&self) -> Tuple4 {
    Tuple4::new(self.c0.y(), self.c1.y(), self.c2.y(), self.c3.y())
  }

  pub fn r2(&self) -> Tuple4 {
    Tuple4::new(self.c0.z(), self.c1.z(), self.c2.z(), self.c3.z())
  }

  pub fn r3(&self) -> Tuple4 {
    Tuple4::new(self.c0.w(), self.c1.w(), self.c2.w(), self.c3.w())
  }

  pub fn transpose(&mut self) {
    let r0 = self.r0();
    let r1 = self.r1();
    let r2 = self.r2();
    let r3 = self.r3();
    self.c0 = r0;
    self.c1 = r1;
    self.c2 = r2;
    self.c3 = r3;
  }

  pub fn transposed(&self) -> Self {
    Matrix4::new(
      self.r0(),
      self.r1(),
      self.r2(),
      self.r3()
    )
  }

  pub fn submatrix(&self, row: usize, column: usize) -> Result<Matrix3, SubmatrixIndexError> {
    if row >= 4 || column >= 4 { 
      Err(SubmatrixIndexError)
    } else {
      let (c0, c1, c2) = match column {
        0 => (self.c1, self.c2, self.c3),
        1 => (self.c0, self.c2, self.c3),
        2 => (self.c0, self.c1, self.c3),
        _ => (self.c0, self.c1, self.c2)
      };
      let (sc0, sc1, sc2) = match row {
        0 => (c0.yzw(), c1.yzw(), c2.yzw()),
        1 => (c0.xzw(), c1.xzw(), c2.xzw()),
        2 => (c0.xyw(), c1.xyw(), c2.xyw()),
        _ => (c0.xyz(), c1.xyz(), c2.xyz()),
      };
      Ok(Matrix3::new(sc0, sc1, sc2))
    }
  }

  pub fn minor(&self, row: usize, column: usize) -> Result<f32, SubmatrixIndexError> {
    match self.submatrix(row, column) {
      Err(e) => Err(e),
      Ok(matrix) => Ok(matrix.determinant())
    }
  }

  pub fn cofactor(&self, row: usize, column: usize) -> Result<f32, SubmatrixIndexError> {
    match self.minor(row, column) {
      Err(e) => Err(e),
      Ok(det) => if (row + column) % 2 != 0 { Ok(-det) } else { Ok(det) }
    }
  }

  pub fn determinant(&self) -> f32 {
    let c00 = self.c0.x() * self.cofactor(0, 0).unwrap();
    let c01 = self.c0.y() * self.cofactor(1, 0).unwrap();
    let c02 = self.c0.z() * self.cofactor(2, 0).unwrap();
    let c03 = self.c0.w() * self.cofactor(3, 0).unwrap();
    c00 + c01 + c02 + c03
  }

  pub fn inverse(&self) -> Result<Matrix4, MatrixInversionError> {
    let determinant = self.determinant();
    if determinant == 0.0 {
      Err(MatrixInversionError)
    } else {
      let inverse = Matrix4::new(
        Tuple4::new(self.cofactor(0, 0).unwrap(), self.cofactor(0, 1).unwrap(), self.cofactor(0, 2).unwrap(), self.cofactor(0, 3).unwrap()),
        Tuple4::new(self.cofactor(1, 0).unwrap(), self.cofactor(1, 1).unwrap(), self.cofactor(1, 2).unwrap(), self.cofactor(1, 3).unwrap()),
        Tuple4::new(self.cofactor(2, 0).unwrap(), self.cofactor(2, 1).unwrap(), self.cofactor(2, 2).unwrap(), self.cofactor(2, 3).unwrap()),
        Tuple4::new(self.cofactor(3, 0).unwrap(), self.cofactor(3, 1).unwrap(), self.cofactor(3, 2).unwrap(), self.cofactor(3, 3).unwrap())
      ) * (1.0 / determinant);
      Ok(inverse)
    }
  }
}

impl Mul<Matrix4> for Matrix4 {
  type Output = Self;

  fn mul(self, rhs: Matrix4) -> Self::Output {
    let r0 = self.r0();
    let r1 = self.r1();
    let r2 = self.r2();
    let r3 = self.r3();
    Self::new(
      Tuple4::new(
        Tuple4::dot(r0, rhs.c0),
        Tuple4::dot(r1, rhs.c0),
        Tuple4::dot(r2, rhs.c0),
        Tuple4::dot(r3, rhs.c0)
      ),
      Tuple4::new(
        Tuple4::dot(r0, rhs.c1),
        Tuple4::dot(r1, rhs.c1),
        Tuple4::dot(r2, rhs.c1),
        Tuple4::dot(r3, rhs.c1)
      ),
      Tuple4::new(
        Tuple4::dot(r0, rhs.c2),
        Tuple4::dot(r1, rhs.c2),
        Tuple4::dot(r2, rhs.c2),
        Tuple4::dot(r3, rhs.c2)
      ),
      Tuple4::new(
        Tuple4::dot(r0, rhs.c3),
        Tuple4::dot(r1, rhs.c3),
        Tuple4::dot(r2, rhs.c3),
        Tuple4::dot(r3, rhs.c3)
      )
    )
  }
}

impl Mul<Tuple4> for Matrix4 {
  type Output = Tuple4;

  fn mul(self, rhs: Tuple4) -> Self::Output {
    Tuple4::new(
      Tuple4::dot(self.r0(), rhs),
      Tuple4::dot(self.r1(), rhs),
      Tuple4::dot(self.r2(), rhs),
      Tuple4::dot(self.r3(), rhs)
    )
  }
}

impl Mul<f32> for Matrix4 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    Self::new(
      self.c0 * rhs,
      self.c1 * rhs,
      self.c2 * rhs,
      self.c3 * rhs
    )
  }
}

#[cfg(test)]
mod tests {
  use super::{Matrix3, Matrix4, SubmatrixIndexError, Tuple4};
  use crate::Tuple3;
  use crate::test_utils::cmp_f32;
  
  fn cmp_tuple4(a: Tuple4, b: Tuple4) -> bool {
    cmp_f32(a.x(), b.x()) &&
    cmp_f32(a.y(), b.y()) &&
    cmp_f32(a.z(), b.z()) && 
    cmp_f32(a.w(), b.w())
  }

  fn cmp_matrix4(a: Matrix4, b: Matrix4) -> bool {
    cmp_tuple4(a.c0, b.c0) &&
    cmp_tuple4(a.c1, b.c1) &&
    cmp_tuple4(a.c2, b.c2) &&
    cmp_tuple4(a.c3, b.c3)
  }

  #[test]
  fn implements_constructor() {
    let mat = Matrix4::new(
      Tuple4::new(1.0, 5.5, 9.0, 13.5),
      Tuple4::new(2.0, 6.5, 10.0, 14.5),
      Tuple4::new(3.0, 7.5, 11.0, 15.5),
      Tuple4::new(4.0, 8.5, 12.0, 16.5)
    );
    assert_eq!(1.0, mat.c0.x());
    assert_eq!(4.0, mat.c3.x());
    assert_eq!(5.5, mat.c0.y());
    assert_eq!(7.5, mat.c2.y());
    assert_eq!(11.0, mat.c2.z());
    assert_eq!(13.5, mat.c0.w());
    assert_eq!(15.5, mat.c2.w());
  }

  #[test]
  fn implements_equality() {
    let mat_a = Matrix4::new(
      Tuple4::new(1.0, 5.0, 9.0, 5.0),
      Tuple4::new(2.0, 6.0, 8.0, 4.0),
      Tuple4::new(3.0, 7.0, 7.0, 3.0),
      Tuple4::new(4.0, 8.0, 6.0, 2.0)
    );
    
    let mat_b = Matrix4::new(
      Tuple4::new(1.0, 5.0, 9.0, 5.0),
      Tuple4::new(2.0, 6.0, 8.0, 4.0),
      Tuple4::new(3.0, 7.0, 7.0, 3.0),
      Tuple4::new(4.0, 8.0, 6.0, 2.0)
    );
    assert_eq!(true, mat_a == mat_b);

    let mat_c = Matrix4::new(
      Tuple4::new(2.0, 6.0, 8.0, 4.0),
      Tuple4::new(3.0, 7.0, 7.0, 3.0),
      Tuple4::new(4.0, 8.0, 7.0, 2.0),
      Tuple4::new(5.0, 9.0, 5.0, 1.0)
    );
    assert_eq!(false, mat_a == mat_c);
  }

  #[test]
  fn implements_mul_matrix() {
    let mat_a = Matrix4::new(
      Tuple4::new(1.0, 5.0, 9.0, 5.0),
      Tuple4::new(2.0, 6.0, 8.0, 4.0),
      Tuple4::new(3.0, 7.0, 7.0, 3.0),
      Tuple4::new(4.0, 8.0, 6.0, 2.0)
    );
    let mat_b = Matrix4::new(
      Tuple4::new(-2.0, 3.0, 4.0, 1.0),
      Tuple4::new(1.0, 2.0, 3.0, 2.0),
      Tuple4::new(2.0, 1.0, 6.0, 7.0),
      Tuple4::new(3.0, -1.0, 5.0, 8.0)
    );

    let mat_r = Matrix4::new(
      Tuple4::new(20.0, 44.0, 40.0, 16.0),
      Tuple4::new(22.0, 54.0, 58.0, 26.0),
      Tuple4::new(50.0, 114.0, 110.0, 46.0),
      Tuple4::new(48.0, 108.0, 102.0, 42.0)
    );
    assert_eq!(mat_r, mat_a * mat_b);
  }

  #[test]
  fn implements_mul_tuple() {
    let mat_a = Matrix4::new(
      Tuple4::new(1.0, 2.0, 8.0, 0.0),
      Tuple4::new(2.0, 4.0, 6.0, 0.0),
      Tuple4::new(3.0, 4.0, 4.0, 0.0),
      Tuple4::new(4.0, 2.0, 1.0, 1.0)
    );
    let b = Tuple4::new(1.0, 2.0, 3.0, 1.0);

    assert_eq!(Tuple4::new(18.0, 24.0, 33.0, 1.0), mat_a * b);
  }

  #[test]
  fn implements_mul_scalar() {
    let mat_a = Matrix4::identity();
    let mat_res = Matrix4::new(
      Tuple4::new(2.5, 0.0, 0.0, 0.0),
      Tuple4::new(0.0, 2.5, 0.0, 0.0),
      Tuple4::new(0.0, 0.0, 2.5, 0.0),
      Tuple4::new(0.0, 0.0, 0.0, 2.5)
    );
    assert_eq!(mat_res, mat_a * 2.5);
  }

  #[test]
  fn implements_identity_constructor() {
    let identity = Matrix4::identity();
    let result = Matrix4::new(
      Tuple4::new(1.0, 0.0, 0.0, 0.0),
      Tuple4::new(0.0, 1.0, 0.0, 0.0),
      Tuple4::new(0.0, 0.0, 1.0, 0.0),
      Tuple4::new(0.0, 0.0, 0.0, 1.0)
    );
    assert!(result == identity);
  }

  #[test]
  fn implements_transpose() {
    let mut mat_a = Matrix4::new(
      Tuple4::new(0.0, 9.0, 1.0, 0.0),
      Tuple4::new(9.0, 8.0, 8.0, 0.0),
      Tuple4::new(3.0, 0.0, 5.0, 5.0),
      Tuple4::new(0.0, 8.0, 3.0, 8.0)
    );
    let mat_a_t = Matrix4::new(
      Tuple4::new(0.0, 9.0, 3.0, 0.0),
      Tuple4::new(9.0, 8.0, 0.0, 8.0),
      Tuple4::new(1.0, 8.0, 5.0, 3.0),
      Tuple4::new(0.0, 0.0, 5.0, 8.0)
    );
    mat_a.transpose();
    assert_eq!(mat_a_t, mat_a);
  }

  #[test]
  fn implements_transposed() {
    let mat_a = Matrix4::new(
      Tuple4::new(0.0, 9.0, 1.0, 0.0),
      Tuple4::new(9.0, 8.0, 8.0, 0.0),
      Tuple4::new(3.0, 0.0, 5.0, 5.0),
      Tuple4::new(0.0, 8.0, 3.0, 8.0)
    );
    let mat_a_t = Matrix4::new(
      Tuple4::new(0.0, 9.0, 3.0, 0.0),
      Tuple4::new(9.0, 8.0, 0.0, 8.0),
      Tuple4::new(1.0, 8.0, 5.0, 3.0),
      Tuple4::new(0.0, 0.0, 5.0, 8.0)
    );
    assert_eq!(mat_a_t, mat_a.transposed());
    assert_eq!(Matrix4::identity(), Matrix4::identity().transposed());
  }

  #[test]
  fn implements_submatrix() {
    let mat_a = Matrix4::new(
      Tuple4::new(-6.0, -8.0, -1.0, -7.0),
      Tuple4::new(1.0, 5.0, 0.0, 1.0),
      Tuple4::new(1.0, 8.0, 8.0, -1.0),
      Tuple4::new(6.0, 6.0, 2.0, 1.0)
    );
    let sub_21 = Matrix3::new(
      Tuple3::new(-6.0, -8.0, -7.0),
      Tuple3::new(1.0, 8.0, -1.0),
      Tuple3::new(6.0, 6.0, 1.0)
    );
    assert_eq!(Ok(sub_21), mat_a.submatrix(2, 1));
    assert_eq!(Err(SubmatrixIndexError), mat_a.submatrix(4, 0));
    assert_eq!(Err(SubmatrixIndexError), mat_a.submatrix(0, 4));
  }

  #[test]
  fn implements_determinant() {
    let mat_a = Matrix4::new(
      Tuple4::new(-2.0, -3.0, 1.0, -6.0),
      Tuple4::new(-8.0, 1.0, 2.0, 7.0),
      Tuple4::new(3.0, 7.0, -9.0, 7.0),
      Tuple4::new(5.0, 3.0, 6.0, -9.0)
    );
    assert_eq!(Ok(690.0), mat_a.cofactor(0, 0));
    assert_eq!(Ok(447.0), mat_a.cofactor(0, 1));
    assert_eq!(Ok(210.0), mat_a.cofactor(0, 2));
    assert_eq!(Ok(51.0), mat_a.cofactor(0, 3));
    assert_eq!(-4071.0, mat_a.determinant())
  }

  #[test]
  fn implements_inverse() {
    let mat_a = Matrix4::new(
      Tuple4::new(-5.0, 1.0, 7.0, 1.0),
      Tuple4::new(2.0, -5.0, 7.0, -3.0),
      Tuple4::new(6.0, 1.0, -6.0, 7.0),
      Tuple4::new(-8.0, 8.0, -7.0, 4.0)
    );
    let mat_a_inverse = mat_a.inverse().unwrap();
    assert_eq!(532.0, mat_a.determinant());
    assert_eq!(Ok(-160.0), mat_a.cofactor(2, 3));
    assert_eq!(Ok(105.0), mat_a.cofactor(3, 2));
    
    let mat_a_inverse_res = Matrix4::new(
      Tuple4::new(0.21804512, -0.8082707, -0.07894737, -0.5225564),
      Tuple4::new(0.45112783, -1.456767, -0.22368422, -0.81390977),
      Tuple4::new(0.24060151, -0.44360903, -0.05263158, -0.3007519),
      Tuple4::new(-0.04511278, 0.52067673, 0.19736843, 0.30639097)
    );
    assert!(cmp_matrix4(mat_a_inverse_res, mat_a_inverse));

    let mat_b = Matrix4::new(
      Tuple4::new(8.0, 7.0, -6.0, -3.0),
      Tuple4::new(-5.0, 5.0, 0.0, 0.0),
      Tuple4::new(9.0, 6.0, 9.0, -9.0),
      Tuple4::new(2.0, 1.0, 6.0, -4.0)
    );
    let mat_b_inverse_res = Matrix4::new(
      Tuple4::new(-0.15384616, -0.07692308, 0.35897437, -0.6923077),
      Tuple4::new(-0.15384616, 0.12307692, 0.35897437, -0.6923077),
      Tuple4::new(-0.2820513, 0.025641026, 0.43589744, -0.7692308),
      Tuple4::new(-0.53846157, 0.03076923, 0.9230769, -1.923077)
    );
    assert!(cmp_matrix4(mat_b_inverse_res, mat_b.inverse().unwrap()));

    let mat_c = Matrix4::new(
      Tuple4::new(9.0, -5.0, -4.0, -7.0),
      Tuple4::new(3.0, -2.0, 9.0, 6.0),
      Tuple4::new(0.0, -6.0, 6.0, 6.0),
      Tuple4::new(9.0, -3.0, 4.0, 2.0)
    );
    let mat_c_inverse_res = Matrix4::new(
      Tuple4::new(-0.040740743, -0.07777778, -0.029012347, 0.17777778),
      Tuple4::new(-0.07777778, 0.033333335, -0.1462963, 0.06666667),
      Tuple4::new(0.14444445, 0.36666667, -0.10925926, -0.26666668),
      Tuple4::new(-0.22222224, -0.33333334, 0.12962964, 0.33333334)
    );
    assert!(cmp_matrix4(mat_c_inverse_res, mat_c.inverse().unwrap()));
  }

  #[test]
  fn math_still_works() {
    let mat_a = Matrix4::new(
      Tuple4::new(3.0, 3.0, -4.0, -6.0),
      Tuple4::new(-9.0, -8.0, 4.0, 5.0),
      Tuple4::new(7.0, 2.0, 4.0, -1.0),
      Tuple4::new(3.0, -9.0, 1.0, 1.0)
    );
    let mat_b = Matrix4::new(
      Tuple4::new(8.0, 3.0, 7.0, 6.0),
      Tuple4::new(2.0, -1.0, 0.0, -2.0),
      Tuple4::new(2.0, 7.0, 5.0, 0.0),
      Tuple4::new(2.0, 0.0, 4.0, 5.0)
    );
    let mat_c = mat_a * mat_b;
    assert!(cmp_matrix4(mat_a, mat_c * mat_b.inverse().unwrap()));
  }
}