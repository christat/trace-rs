use std::ops::Mul;
use crate::Matrix2;
use crate::Tuple3;
use crate::errors::SubmatrixIndexError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3 {
  pub c0: Tuple3,
  pub c1: Tuple3,
  pub c2: Tuple3
}

impl Matrix3 {
  pub fn new(c0: Tuple3, c1: Tuple3, c2: Tuple3) -> Self {
    Self { c0: c0, c1: c1, c2: c2 }
  }

  pub fn identity() -> Self {
    Self::new(
      Tuple3::new(1.0, 0.0, 0.0),
      Tuple3::new(0.0, 1.0, 0.0),
      Tuple3::new(0.0, 0.0, 1.0),
    )
  }

  pub fn r0(&self) -> Tuple3 {
    Tuple3::new(self.c0.x(), self.c1.x(), self.c2.x())
  }

  pub fn r1(&self) -> Tuple3 {
    Tuple3::new(self.c0.y(), self.c1.y(), self.c2.y())
  }

  pub fn r2(&self) -> Tuple3 {
    Tuple3::new(self.c0.z(), self.c1.z(), self.c2.z())
  }

  pub fn transpose(&mut self) {
    let r0 = self.r0();
    let r1 = self.r1();
    let r2 = self.r2();
    self.c0 = r0;
    self.c1 = r1;
    self.c2 = r2;
  }

  pub fn transposed(&self) -> Self {
    Matrix3::new(
      self.r0(),
      self.r1(),
      self.r2()
    )
  }

  pub fn submatrix(&self, row: usize, column: usize) -> Result<Matrix2, SubmatrixIndexError> {
    if row >= 3 || column >= 3 { 
      Err(SubmatrixIndexError)
    } else {
      let (c0, c1) = match column {
        0 => (self.c1, self.c2),
        1 => (self.c0, self.c2),
        _ => (self.c0, self.c1)
      };
      let (sc0, sc1) = match row {
        0 => (c0.yz(), c1.yz()),
        1 => (c0.xz(), c1.xz()),
        _ => (c0.xy(), c1.xy()),
      };
      Ok(Matrix2::new(sc0, sc1))
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
    c00 + c01 + c02
  }
}

impl Mul<Matrix3> for Matrix3 {
  type Output = Self;

  fn mul(self, rhs: Matrix3) -> Self::Output {
    let r0 = self.r0();
    let r1 = self.r1();
    let r2 = self.r2();
    Self::new(
      Tuple3::new(
        Tuple3::dot(r0, rhs.c0),
        Tuple3::dot(r1, rhs.c0),
        Tuple3::dot(r2, rhs.c0)
      ),
      Tuple3::new(
        Tuple3::dot(r0, rhs.c1),
        Tuple3::dot(r1, rhs.c1),
        Tuple3::dot(r2, rhs.c1)
      ),
      Tuple3::new(
        Tuple3::dot(r0, rhs.c2),
        Tuple3::dot(r1, rhs.c2),
        Tuple3::dot(r2, rhs.c2)
      )
    )
  }
}

impl Mul<Tuple3> for Matrix3 {
  type Output = Tuple3;

  fn mul(self, rhs: Tuple3) -> Self::Output {
    Tuple3::new(
      Tuple3::dot(self.r0(), rhs),
      Tuple3::dot(self.r1(), rhs),
      Tuple3::dot(self.r2(), rhs)
    )
  }
}

impl Mul<f32> for Matrix3 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    Self::new(
      self.c0 * rhs,
      self.c1 * rhs,
      self.c2 * rhs
    )
  }
}

#[cfg(test)]
mod tests {
  mod methods {
    use crate::{Matrix2, Matrix3, SubmatrixIndexError, Tuple2, Tuple3};

    #[test]
    fn constructor() {
      let mat = Matrix3::new(
        Tuple3::new(1.0, 5.5, 9.0),
        Tuple3::new(2.0, 6.5, 10.0),
        Tuple3::new(3.0, 7.5, 11.0)
      );
      assert_eq!(1.0, mat.c0.x());
      assert_eq!(5.5, mat.c0.y());
      assert_eq!(7.5, mat.c2.y());
      assert_eq!(11.0, mat.c2.z());
    }

    #[test]
    fn transpose() {
      let mut mat_a = Matrix3::new(
        Tuple3::new(0.0, 9.0, 1.0),
        Tuple3::new(9.0, 8.0, 8.0),
        Tuple3::new(3.0, 0.0, 5.0)
      );
      let mat_a_t = Matrix3::new(
        Tuple3::new(0.0, 9.0, 3.0),
        Tuple3::new(9.0, 8.0, 0.0),
        Tuple3::new(1.0, 8.0, 5.0),
      );
      mat_a.transpose();
      assert_eq!(mat_a_t, mat_a);
    }

    #[test]
    fn transposed() {
      let mat_a = Matrix3::new(
        Tuple3::new(0.0, 9.0, 1.0),
        Tuple3::new(9.0, 8.0, 8.0),
        Tuple3::new(3.0, 0.0, 5.0)
      );
      let mat_a_t = Matrix3::new(
        Tuple3::new(0.0, 9.0, 3.0),
        Tuple3::new(9.0, 8.0, 0.0),
        Tuple3::new(1.0, 8.0, 5.0)
      );
      assert_eq!(mat_a_t, mat_a.transposed());
      assert_eq!(Matrix3::identity(), Matrix3::identity().transposed());
    }

    #[test]
    fn submatrix() {
      let mat_a = Matrix3::new(
        Tuple3::new(1.0, -3.0, 0.0),
        Tuple3::new(5.0, 2.0, 6.0),
        Tuple3::new(0.0, 7.0, -3.0)
      );
      let sub_02 = Matrix2::new(
        Tuple2::new(-3.0, 0.0),
        Tuple2::new(2.0, 6.0)
      );
      assert_eq!(Ok(sub_02), mat_a.submatrix(0, 2));
      assert_eq!(Err(SubmatrixIndexError), mat_a.submatrix(3, 0));
      assert_eq!(Err(SubmatrixIndexError), mat_a.submatrix(0, 3));
    }

    #[test]
    fn minor() {
      let mat_a = Matrix3::new(
        Tuple3::new(3.0, 2.0, 6.0),
        Tuple3::new(5.0, -1.0, -1.0),
        Tuple3::new(0.0, -7.0, 5.0)
      );
      assert_eq!(Ok(25.0), mat_a.minor(1, 0));
      assert_eq!(Err(SubmatrixIndexError), mat_a.minor(3, 0));
      assert_eq!(Err(SubmatrixIndexError), mat_a.minor(0, 3));
    }

    #[test]
    fn cofactor() {
      let mat_a = Matrix3::new(
        Tuple3::new(3.0, 2.0, 6.0),
        Tuple3::new(5.0, -1.0, -1.0),
        Tuple3::new(0.0, -7.0, 5.0)
      );
      assert_eq!(Ok(-12.0), mat_a.cofactor(0, 0));
      assert_eq!(Ok(-25.0), mat_a.cofactor(1, 0));
      assert_eq!(Err(SubmatrixIndexError), mat_a.cofactor(3, 0));
      assert_eq!(Err(SubmatrixIndexError), mat_a.cofactor(0, 3));
    }

    #[test]
    fn determinant() {
      let mat_a = Matrix3::new(
        Tuple3::new(1.0, -5.0, 2.0),
        Tuple3::new(2.0, 8.0, 6.0),
        Tuple3::new(6.0, -4.0, 4.0)
      );
      assert_eq!(Ok(56.0), mat_a.cofactor(0, 0));
      assert_eq!(Ok(12.0), mat_a.cofactor(0, 1));
      assert_eq!(Ok(-46.0), mat_a.cofactor(0, 2));
      assert_eq!(-196.0, mat_a.determinant());
    }
  }

  mod traits {
    use crate::{Matrix3, Tuple3};

    #[test]
    fn equality() {
      let mat_a = Matrix3::new(
        Tuple3::new(1.0, 5.0, 9.0),
        Tuple3::new(2.0, 6.0, 8.0),
        Tuple3::new(3.0, 7.0, 7.0)
      );
      
      let mat_b = Matrix3::new(
        Tuple3::new(1.0, 5.0, 9.0),
        Tuple3::new(2.0, 6.0, 8.0),
        Tuple3::new(3.0, 7.0, 7.0)
      );
      assert_eq!(true, mat_a == mat_b);

      let mat_c = Matrix3::new(
        Tuple3::new(2.0, 6.0, 8.0),
        Tuple3::new(3.0, 7.0, 7.0),
        Tuple3::new(4.0, 8.0, 7.0)
      );
      assert_eq!(false, mat_a == mat_c);
    }

    #[test]
    fn mul_matrix() {
      let mat_a = Matrix3::new(
        Tuple3::new(1.0, 5.0, 9.0),
        Tuple3::new(2.0, 6.0, 8.0),
        Tuple3::new(3.0, 7.0, 7.0)
      );
      let mat_b = Matrix3::new(
        Tuple3::new(-2.0, 3.0, 4.0),
        Tuple3::new(1.0, 2.0, 3.0),
        Tuple3::new(2.0, 1.0, 6.0)
      );

      let mat_r = Matrix3::new(
        Tuple3::new(16.0, 36.0, 34.0),
        Tuple3::new(14.0, 38.0, 46.0),
        Tuple3::new(22.0, 58.0, 68.0)
      );
      assert_eq!(mat_r, mat_a * mat_b);
    }

    #[test]
    fn mul_tuple() {
      let mat_a = Matrix3::new(
        Tuple3::new(1.0, 2.0, 8.0),
        Tuple3::new(2.0, 4.0, 6.0),
        Tuple3::new(3.0, 4.0, 4.0)
      );
      let b = Tuple3::new(1.0, 2.0, 3.0);

      assert_eq!(Tuple3::new(14.0, 22.0, 32.0), mat_a * b);
    }

    #[test]
    fn mul_scalar() {
      let mat_a = Matrix3::identity();
      let mat_res = Matrix3::new(
        Tuple3::new(2.5, 0.0, 0.0),
        Tuple3::new(0.0, 2.5, 0.0),
        Tuple3::new(0.0, 0.0, 2.5)
      );
      assert_eq!(mat_res, mat_a * 2.5);
    }
  }
}