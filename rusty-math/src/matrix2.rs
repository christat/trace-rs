use std::ops::Mul;
use crate::Tuple2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2 {
  pub c0: Tuple2,
  pub c1: Tuple2
}

impl Matrix2 {
  pub fn new(c0: Tuple2, c1: Tuple2) -> Self {
    Self { c0: c0, c1: c1 }
  }

  pub fn identity() -> Self {
    Self::new(
      Tuple2::new(1.0, 0.0),
      Tuple2::new(0.0, 1.0)
    )
  }

  pub fn r0(&self) -> Tuple2 {
    Tuple2::new(self.c0.x(), self.c1.x())
  }

  pub fn r1(&self) -> Tuple2 {
    Tuple2::new(self.c0.y(), self.c1.y())
  }

  pub fn transpose(&mut self) {
    let r0 = self.r0();
    let r1 = self.r1();
    self.c0 = r0;
    self.c1 = r1;
  }

  pub fn transposed(&self) -> Self {
    Matrix2::new(
      self.r0(),
      self.r1()
    )
  }

  pub fn determinant(&self) -> f32 {
    self.c0.x() * self.c1.y() -
    self.c1.x() * self.c0.y()
  }
}

impl Mul<Matrix2> for Matrix2 {
  type Output = Self;

  fn mul(self, rhs: Matrix2) -> Self::Output {
    let r0 = self.r0();
    let r1 = self.r1();
    Matrix2 {
      c0: Tuple2::new(
        Tuple2::dot(r0, rhs.c0),
        Tuple2::dot(r1, rhs.c0)
      ),
      c1: Tuple2::new(
        Tuple2::dot(r0, rhs.c1),
        Tuple2::dot(r1, rhs.c1)
      )
    }
  }
}

impl Mul<Tuple2> for Matrix2 {
  type Output = Tuple2;

  fn mul(self, rhs: Tuple2) -> Self::Output {
    Tuple2::new(
      Tuple2::dot(self.r0(), rhs),
      Tuple2::dot(self.r1(), rhs)
    )
  }
}

impl Mul<f32> for Matrix2 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    Self::new(
      self.c0 * rhs,
      self.c1 * rhs
    )
  }
}

#[cfg(test)]
mod tests {
  mod methods {
    use crate::{Matrix2, Tuple2};

    #[test]
    fn constructor() {
      let mat = Matrix2::new(
        Tuple2::new(1.0, 5.5),
        Tuple2::new(2.0, 6.5)
      );
      assert_eq!(1.0, mat.c0.x());
      assert_eq!(5.5, mat.c0.y());
      assert_eq!(2.0, mat.c1.x());
      assert_eq!(6.5, mat.c1.y());
    }

    #[test]
    fn identity() {
      let identity = Matrix2::identity();
      let result = Matrix2::new(
        Tuple2::new(1.0, 0.0),
        Tuple2::new(0.0, 1.0)
      );
      assert!(result == identity);
    }

    #[test]
    fn transpose() {
      let mut mat_a = Matrix2::new(
        Tuple2::new(0.0, 9.0),
        Tuple2::new(9.0, 8.0)
      );
      let mat_a_t = Matrix2::new(
        Tuple2::new(0.0, 9.0),
        Tuple2::new(9.0, 8.0),
      );
      mat_a.transpose();
      assert_eq!(mat_a_t, mat_a);
    }

    #[test]
    fn transposed() {
      let mat_a = Matrix2::new(
        Tuple2::new(0.0, 9.0),
        Tuple2::new(9.0, 8.0)
      );
      let mat_a_t = Matrix2::new(
        Tuple2::new(0.0, 9.0),
        Tuple2::new(9.0, 8.0)
      );
      assert_eq!(mat_a_t, mat_a.transposed());
      assert_eq!(Matrix2::identity(), Matrix2::identity().transposed());
    }

    #[test]
    fn determinant() {
      let mat_a = Matrix2::new(
        Tuple2::new(1.0, -3.0),
        Tuple2::new(5.0, 2.0)
      );
      assert_eq!(17.0, mat_a.determinant());
    }
  }

  mod traits {
    use crate::{Matrix2, Tuple2};

    #[test]
    fn equality() {
      let mat_a = Matrix2::new(
        Tuple2::new(1.0, 5.0),
        Tuple2::new(2.0, 6.0)
      );
      
      let mat_b = Matrix2::new(
        Tuple2::new(1.0, 5.0),
        Tuple2::new(2.0, 6.0)
      );
      assert_eq!(true, mat_a == mat_b);

      let mat_c = Matrix2::new(
        Tuple2::new(2.0, 6.0),
        Tuple2::new(3.0, 7.0)
      );
      assert_eq!(false, mat_a == mat_c);
    }

    #[test]
    fn mul_matrix() {
      let mat_a = Matrix2::new(
        Tuple2::new(1.0, 5.0),
        Tuple2::new(2.0, 6.0)
      );
      let mat_b = Matrix2::new(
        Tuple2::new(-2.0, 3.0),
        Tuple2::new(1.0, 2.0)
      );

      let mat_r = Matrix2::new(
        Tuple2::new(4.0, 8.0),
        Tuple2::new(5.0, 17.0)
      );
      assert_eq!(mat_r, mat_a * mat_b);
    }

    #[test]
    fn mul_tuple() {
      let mat_a = Matrix2::new(
        Tuple2::new(1.0, 2.0),
        Tuple2::new(2.0, 4.0)
      );
      let b = Tuple2::new(1.0, 2.0);

      assert_eq!(Tuple2::new(5.0, 10.0), mat_a * b);
    }

    #[test]
    fn mul_scalar() {
      let mat_a = Matrix2::identity();
      let mat_res = Matrix2::new(
        Tuple2::new(2.5, 0.0),
        Tuple2::new(0.0, 2.5)
      );
      assert_eq!(mat_res, mat_a * 2.5);
    }
  }
}