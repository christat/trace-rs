use yaac::Tuple4;

use crate::components::{Material, Position, Shape, Transform};
use crate::systems::IntersectionRecord;
use crate::Ray;

pub fn intersect<'a>(
  radius: f32,
  ray: &Ray,
  position: &'a Position,
  shape: &'a Shape,
  transform: &'a Transform,
  material: &'a Material,
) -> Option<Vec<IntersectionRecord<'a>>> {
  let inv_transform = match transform.0.inverse() {
    Ok(inv) => inv,
    Err(_) => return None,
  };
  let ray_tr = ray.transform(inv_transform);
  let vec_sphere_ray = ray_tr.origin - position.0;

  let a = Tuple4::dot(ray_tr.direction, ray_tr.direction);
  let b = 2.0 * Tuple4::dot(ray_tr.direction, vec_sphere_ray);
  let c = Tuple4::dot(vec_sphere_ray, vec_sphere_ray) - radius.powi(2);
  let discriminant = b.powi(2) - 4.0 * a * c;

  if discriminant < 0.0 {
    None
  } else {
    let sqrt_discriminant = f32::sqrt(discriminant);
    let inv_denominator = 1.0 / (2.0 * a);
    let t1 = (-b - sqrt_discriminant) * inv_denominator;
    let t2 = (-b + sqrt_discriminant) * inv_denominator;
    Some(vec![
      IntersectionRecord {
        t: t1,
        position,
        shape,
        transform,
        material,
      },
      IntersectionRecord {
        t: t2,
        position,
        shape,
        transform,
        material,
      },
    ])
  }
}

pub fn normal_at(point: &Tuple4, pos: &Position, tr: &Transform) -> Tuple4 {
  let mut inv_transform = tr.0.inverse().unwrap();
  let obj_n = inv_transform * (*point) - pos.0;

  inv_transform.transpose();
  let mut world_n = inv_transform * obj_n;
  world_n.set_w(0.0);
  world_n.normalize();
  world_n
}

#[cfg(test)]
mod tests {
  use crate::components::{Material, Position, Shape, Transform};
  use crate::Ray;
  use crate::systems::IntersectionRecord;
  use super::{intersect, normal_at};
  use yaac::{test_utils, Matrix4, Tuple4};

  #[test]
  fn intersect_sphere() {
    let position = &Position::default();
    let radius = 1.0;
    let shape = &Shape::Sphere { radius };
    let transform = &Transform(Matrix4::identity());
    let material = &Material::default();

    // intersecting through middle of sphere
    let r = &Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
    let res = Some(vec![
      IntersectionRecord {
        t: 4.0,
        position,
        shape,
        transform,
        material,
      },
      IntersectionRecord {
        t: 6.0,
        position,
        shape,
        transform,
        material,
      },
    ]);
    assert_eq!(
      res,
      intersect(radius, r, position, shape, transform, material)
    );

    // tangent intersection
    let r = &Ray::new(Tuple4::point(0.0, 1.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
    let res = Some(vec![
      IntersectionRecord {
        t: 5.0,
        position,
        shape,
        transform,
        material,
      },
      IntersectionRecord {
        t: 5.0,
        position,
        shape,
        transform,
        material,
      },
    ]);
    assert_eq!(
      res,
      intersect(radius, r, position, shape, transform, material)
    );

    // ray miss
    let r = &Ray::new(Tuple4::point(0.0, 2.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
    assert_eq!(
      None,
      intersect(radius, r, position, shape, transform, material)
    );

    // ray originated inside sphere
    let r = &Ray::new(Tuple4::point(0.0, 0.0, 0.0), Tuple4::vector(0.0, 0.0, 1.0));
    let res = Some(vec![
      IntersectionRecord {
        t: -1.0,
        position,
        shape,
        transform,
        material,
      },
      IntersectionRecord {
        t: 1.0,
        position,
        shape,
        transform,
        material,
      },
    ]);
    assert_eq!(
      res,
      intersect(radius, r, position, shape, transform, material)
    );

    // sphere is behind ray
    let r = &Ray::new(Tuple4::point(0.0, 0.0, 5.0), Tuple4::vector(0.0, 0.0, 1.0));
    let res = Some(vec![
      IntersectionRecord {
        t: -6.0,
        position,
        shape,
        transform,
        material,
      },
      IntersectionRecord {
        t: -4.0,
        position,
        shape,
        transform,
        material,
      },
    ]);
    assert_eq!(
      res,
      intersect(radius, r, position, shape, transform, material)
    );

    // scaled sphere
    let r = &Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
    let transform = &Transform(Matrix4::scaling(2.0, 2.0, 2.0));
    let res = Some(vec![
      IntersectionRecord {
        t: 3.0,
        position,
        shape,
        transform,
        material,
      },
      IntersectionRecord {
        t: 7.0,
        position,
        shape,
        transform,
        material,
      },
    ]);
    assert_eq!(
      res,
      intersect(radius, r, position, shape, transform, material)
    );

    // translated sphere
    let r = &Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
    let transform = &Transform(Matrix4::translation(5.0, 0.0, 0.0));
    assert_eq!(
      None,
      intersect(radius, r, position, shape, transform, material)
    );
  }

  #[test]
  fn sphere_normal_at() {
    let position = Position(Tuple4::point(0.0, 0.0, 0.0));
    let transform = Transform(Matrix4::identity());
    // point along x-axis
    let point = Tuple4::point(1.0, 0.0, 0.0);
    let normal = normal_at(&point, &position, &transform);
    assert_eq!(Tuple4::vector(1.0, 0.0, 0.0), normal);

    // point along y-axis
    let point = Tuple4::point(0.0, 1.0, 0.0);
    let normal = normal_at(&point, &position, &transform);
    assert_eq!(Tuple4::vector(0.0, 1.0, 0.0), normal);

    // point along z-axis
    let point = Tuple4::point(0.0, 0.0, 1.0);
    let normal = normal_at(&point, &position, &transform);
    assert_eq!(Tuple4::vector(0.0, 0.0, 1.0), normal);

    // non-axis point
    let coord = f32::sqrt(3.0) / 3.0;
    let point = Tuple4::point(coord, coord, coord);
    let normal = normal_at(&point, &position, &transform);
    assert!(test_utils::cmp_tuple4(
      Tuple4::vector(coord, coord, coord),
      normal
    ));
    assert!(test_utils::cmp_tuple4(normal.normalized(), normal));

    // sphere with translation transform
    let transform = Transform(Matrix4::translation(0.0, 1.0, 0.0));
    let point = Tuple4::point(0.0, 1.70711, -0.70711);
    let normal = normal_at(&point, &position, &transform);
    assert!(test_utils::cmp_tuple4(
      Tuple4::vector(0.0, 0.70711, -0.70711),
      normal
    ));

    // sphere with complex transform
    let transform =
      Transform(Matrix4::scaling(1.0, 0.5, 1.0) * Matrix4::rotation_z(std::f32::consts::PI / 5.0));
    let coord = f32::sqrt(2.0) / 2.0;
    let point = Tuple4::point(0.0, coord, coord);
    let normal = normal_at(&point, &position, &transform);
    assert!(test_utils::cmp_tuple4(
      Tuple4::vector(0.0, 0.97014, -0.24254),
      normal
    ));
  }
}
