use specs::{Join, ReadStorage, System};

use rm::{Tuple4};
use crate::common::Ray;
use crate::components::{Position, Shape, SphereShape, Transform};

#[derive(Debug)]
struct IntersectionRecord {
  t: f32,
/*
  material: Material,
  position: Position,
  shape: Shape,
  transform: Transform
*/
}

pub struct Intersection;

impl<'a> System<'a> for Intersection {
  type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Shape>, ReadStorage<'a, Transform>);

  fn run(&mut self, (position, shape, transform): Self::SystemData) {
    // TODO FIGURE OUT RAY GENERATION/PIXEL THREADING
    let ray = Ray::new(Tuple4::point(0.0, 0.0, 0.0), Tuple4::vector(0.0, 0.0, 1.0));

    let mut intersection_records: Vec<IntersectionRecord> = vec![];

    for (position, shape, transform) in (&position, &shape, &transform).join() {
      let intersection = match shape {
        Shape::Sphere(sphere_shape) => intersects_sphere(&ray, position, sphere_shape, transform),
        _ => {
          println!("INTERSECTION SKIPPED! MISSING SHAPE INTERSECTOR FOR {:?}", shape);
          None
        }
      };
      if intersection.is_some() {
        intersection_records.append(&mut intersection.unwrap())
      }
    }

    // TODO HAND OVER INTERSECTION_RECORDS TO BRDF FUNCTION (?)

  }
}

/* fn closest_hit(vec: &[IntersectionRecord]) -> Option<IntersectionRecord> {

} */

fn intersects_sphere(ray: &Ray, position: &Position, shape: &SphereShape, transform: &Transform) -> Option<Vec<IntersectionRecord>> {
    let inv_transform = match transform.0.inverse() {
      Ok(inv) => inv,
      Err(_) => return None,
    };
    let ray_tr = ray.transform(inv_transform);
    let vec_sphere_ray = ray_tr.origin - position.0;

    let a = Tuple4::dot(ray_tr.direction, ray_tr.direction);
    let b = 2.0 * Tuple4::dot(ray_tr.direction, vec_sphere_ray);
    let c = Tuple4::dot(vec_sphere_ray, vec_sphere_ray) - shape.radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
      None
    } else {
      let sqrt_discriminant = f32::sqrt(discriminant);
      let inv_denominator = 1.0 / (2.0 * a);
      let t1 = (-b - sqrt_discriminant) * inv_denominator;
      let t2 = (-b + sqrt_discriminant) * inv_denominator;
      Some(vec![
        IntersectionRecord { t: t1 },
        IntersectionRecord { t: t2 },
      ])
    }
}