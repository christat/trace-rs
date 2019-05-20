extern crate rusty_math as rm;

use crate::intersection::{Intersect, IntersectionRecord};
use crate::material::{Material};
use crate::ray::Ray;
use rm::{Matrix4, Tuple4};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
  origin: Tuple4,
  radius: f32,
  transform: Option<Matrix4>,
  material: Option<Material>
}

impl Sphere {
  pub fn new(origin: Tuple4, radius: f32, material: Material) -> Self {
    Self {
      origin: origin,
      radius: radius,
      transform: None,
      material: Some(material)
    }
  }

  pub fn unit() -> Self {
    Self {
      origin: Tuple4::point(0.0, 0.0, 0.0),
      radius: 1.0,
      transform: None,
      material: None
    }
  }

  pub fn set_transform(&mut self, t: Matrix4) {
    self.transform = Some(t);
  }

  pub fn set_material(&mut self, mat: Material) {
    self.material = Some(mat);
  }

  pub fn normal_at(&self, p: Tuple4) -> Tuple4 {
    let mut inv_transform = self.get_transform().inverse().unwrap();
    let obj_n = inv_transform * p - self.origin;

    inv_transform.transpose();
    let mut world_n = inv_transform * obj_n;
    world_n.set_w(0.0);
    world_n.normalize();
    world_n
  }

  pub fn get_transform(&self) -> Matrix4 {
    match self.transform {
      Some(t) => t,
      None => Matrix4::identity(),
    }
  }

  pub fn get_material(&self) -> Material {
    match self.material {
      Some(m) => m,
      None => Material::default()
    }
  }
}

impl Intersect for Sphere {
  fn intersects(&self, r: Ray) -> Option<Vec<IntersectionRecord<Self>>> {
    let transform = self.get_transform();
    let inv_transform = match transform.inverse() {
      Ok(inv) => inv,
      Err(_) => return None,
    };
    let ray_tr = r.transform(inv_transform);
    let vec_sphere_ray = ray_tr.origin - self.origin;

    let a = Tuple4::dot(ray_tr.direction, ray_tr.direction);
    let b = 2.0 * Tuple4::dot(ray_tr.direction, vec_sphere_ray);
    let c = Tuple4::dot(vec_sphere_ray, vec_sphere_ray) - self.radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
      None
    } else {
      let sqrt_discriminant = f32::sqrt(discriminant);
      let inv_denominator = 1.0 / (2.0 * a);
      let t1 = (-b - sqrt_discriminant) * inv_denominator;
      let t2 = (-b + sqrt_discriminant) * inv_denominator;
      Some(vec![
        IntersectionRecord { t: t1, o: &self },
        IntersectionRecord { t: t2, o: &self },
      ])
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Material, Matrix4, Sphere, Tuple4};
  use crate::color::Color;
  use crate::intersection::{hit, Intersect, IntersectionRecord};
  use crate::ray::Ray;
  use std::f32::consts::{PI};
  extern crate rusty_math;
  use rusty_math::test_utils;

  #[test]
  fn implements_constructor() {
    let o = Tuple4::point(0.0, 0.0, 0.0);
    let r = 2.5;
    let mat = Material::default();
    assert_eq!(
      Sphere {
        origin: o,
        radius: r,
        transform: None,
        material: Some(mat)
      },
      Sphere::new(o, r, mat)
    );
  }

  #[test]
  fn implements_set_transform() {
    let mut s = Sphere::new(Tuple4::point(1.0, 2.0, 3.0), 4.0, Material::default());
    assert_eq!(None, s.transform);

    let t = Matrix4::translation(2.0, 3.0, 4.0);
    s.set_transform(t);
    assert_eq!(t, s.transform.unwrap());
  }

  #[test]
  fn implements_set_material() {
    let mut s = Sphere::unit();
    let mat = Material::new(Color::new(0.0, 1.0, 2.0), 3.0, 4.0, 5.0, 6.0);
    s.set_material(mat);
    assert_eq!(mat, s.material.unwrap());
  }

  #[test]
  fn implements_intersects() {
    // intersecting through middle of sphere
    let r = Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
    let s = Sphere::unit();
    let res = Some(vec![
      IntersectionRecord { t: 4.0, o: &s },
      IntersectionRecord { t: 6.0, o: &s },
    ]);
    assert_eq!(res, s.intersects(r));

    // tangent intersection
    let r = Ray::new(Tuple4::point(0.0, 1.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
    let res = Some(vec![
      IntersectionRecord { t: 5.0, o: &s },
      IntersectionRecord { t: 5.0, o: &s },
    ]);
    assert_eq!(res, s.intersects(r));

    // ray miss
    let r = Ray::new(Tuple4::point(0.0, 2.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
    assert_eq!(None, s.intersects(r));

    // ray originated inside sphere
    let r = Ray::new(Tuple4::point(0.0, 0.0, 0.0), Tuple4::vector(0.0, 0.0, 1.0));
    let res = Some(vec![
      IntersectionRecord { t: -1.0, o: &s },
      IntersectionRecord { t: 1.0, o: &s },
    ]);
    assert_eq!(res, s.intersects(r));

    // sphere is behind ray
    let r = Ray::new(Tuple4::point(0.0, 0.0, 5.0), Tuple4::vector(0.0, 0.0, 1.0));
    let res = Some(vec![
      IntersectionRecord { t: -6.0, o: &s },
      IntersectionRecord { t: -4.0, o: &s },
    ]);
    assert_eq!(res, s.intersects(r));

    // scaled sphere
    let mut s = Sphere::unit();
    let r = Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
    s.set_transform(Matrix4::scaling(2.0, 2.0, 2.0));
    let res = Some(vec![
      IntersectionRecord { t: 3.0, o: &s },
      IntersectionRecord { t: 7.0, o: &s },
    ]);
    assert_eq!(res, s.intersects(r));

    // translated sphere
    let mut s = Sphere::unit();
    let r = Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
    s.set_transform(Matrix4::translation(5.0, 0.0, 0.0));
    assert_eq!(None, s.intersects(r));
  }

  #[test]
  fn test_intersection_hit() {
    // smallest
    let s = Sphere::unit();
    let i1 = IntersectionRecord { t: 1.0, o: &s };
    let i2 = IntersectionRecord { t: 2.0, o: &s };
    assert_eq!(&i1, hit(&vec![i1, i2]).unwrap());

    // filter negatives
    let i1 = IntersectionRecord { t: -1.0, o: &s };
    let i2 = IntersectionRecord { t: 1.0, o: &s };
    assert_eq!(&i2, hit(&vec![i1, i2]).unwrap());

    // no intersection in frustum
    let i1 = IntersectionRecord { t: -2.0, o: &s };
    let i2 = IntersectionRecord { t: -1.0, o: &s };
    assert_eq!(None, hit(&vec![i1, i2]));

    // smallest positive
    let i1 = IntersectionRecord { t: 5.0, o: &s };
    let i2 = IntersectionRecord { t: 7.0, o: &s };
    let i3 = IntersectionRecord { t: -3.0, o: &s };
    let i4 = IntersectionRecord { t: 2.0, o: &s };
    assert_eq!(&i4, hit(&vec![i1, i2, i3, i4]).unwrap());
  }

  #[test]
  fn implements_normal_at() {
    let s = Sphere::unit();
    let n = s.normal_at(Tuple4::point(1.0, 0.0, 0.0));
    assert_eq!(Tuple4::vector(1.0, 0.0, 0.0), n);

    let n = s.normal_at(Tuple4::point(0.0, 1.0, 0.0));
    assert_eq!(Tuple4::vector(0.0, 1.0, 0.0), n);

    let n = s.normal_at(Tuple4::point(0.0, 0.0, 1.0));
    assert_eq!(Tuple4::vector(0.0, 0.0, 1.0), n);

    let coord = f32::sqrt(3.0) / 3.0;
    let n = s.normal_at(Tuple4::point(coord, coord, coord));
    assert!(test_utils::cmp_tuple4(Tuple4::vector(coord, coord, coord), n));
    assert!(test_utils::cmp_tuple4(n.normalized(), n));
  }

  #[test]
  fn test_normal_at_with_transforms() {
    let mut s = Sphere::unit();
    s.set_transform(Matrix4::translation(0.0, 1.0, 0.0));
    assert!(test_utils::cmp_tuple4(Tuple4::vector(0.0, 0.70711, -0.70711), s.normal_at(Tuple4::point(0.0, 1.70711, -0.70711))));

    s.set_transform(Matrix4::scaling(1.0, 0.5, 1.0) * Matrix4::rotation_z(PI / 5.0));
    let coord = f32::sqrt(2.0) / 2.0;
    assert!(test_utils::cmp_tuple4(Tuple4::vector(0.0, 0.97014, -0.24254), s.normal_at(Tuple4::point(0.0, coord, coord))));
  }
}
