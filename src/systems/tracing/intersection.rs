use specs::{Join, Read, ReadStorage};
use std::cmp::Ordering;

use crate::components::{Color, Material, Position, Shape, Transform};
use crate::resources::Lights;
use crate::systems::LightingSubsystem;
use crate::utils::IntersectionRecord;
use crate::Ray;

pub struct IntersectionSubsystem;

impl IntersectionSubsystem {
  pub fn run<'a>(
    ray: &Ray,
    lights: &Read<'a, Lights>,
    positions: &ReadStorage<'a, Position>,
    shapes: &ReadStorage<'a, Shape>,
    transforms: &ReadStorage<'a, Transform>,
    materials: &ReadStorage<'a, Material>,
  ) -> Option<Color> {
    let mut intersection_records: Vec<IntersectionRecord> = vec![];

    // TODO - build BVH tree and intersect here
    for (pos, shape, trans, mat) in (positions, shapes, transforms, materials).join() {
      let intersection = shape.intersect(ray, pos, shape, trans, mat);
      if intersection.is_some() {
        intersection_records.append(&mut intersection.unwrap())
      }
    }

    let hit_record = Self::nearest_hit(&intersection_records);
    if hit_record.is_none() {
      return None;
    }

    let hit = hit_record.unwrap();
    let point = ray.point_at(hit.t);
    let eye = -ray.direction;
    let normal = hit.shape.normal_at(&point, hit.position, hit.transform);
    Some(LightingSubsystem::run(
      hit.material,
      &lights.0,
      &point,
      &eye,
      &normal,
    ))
  }

  fn nearest_hit<'a>(records: &'a [IntersectionRecord]) -> Option<&'a IntersectionRecord<'a>> {
    if records.len() == 0 {
      return None;
    }
    records
      .iter()
      .filter(|ir| ir.t.is_sign_positive())
      .min_by(|ir1, ir2| ir1.t.partial_cmp(&ir2.t).unwrap_or(Ordering::Equal))
  }
}

#[cfg(test)]
mod tests {
  use super::IntersectionSubsystem;
  use crate::components::{Material, Position, Shape, Transform};
  use crate::utils::IntersectionRecord;

  #[test]
  fn nearest_hit() {
    let position = &Position::origin();
    let radius = 1.0;
    let shape = &Shape::Sphere { radius };
    let transform = &Transform::default();
    let material = &Material::default();
    // smallest
    let i1 = IntersectionRecord {
      t: 1.0,
      position,
      shape,
      transform,
      material,
    };
    let i2 = IntersectionRecord {
      t: 2.0,
      position,
      shape,
      transform,
      material,
    };
    assert_eq!(
      &i1,
      IntersectionSubsystem::nearest_hit(&vec![i1, i2]).unwrap()
    );

    // filter negatives
    let i1 = IntersectionRecord {
      t: -1.0,
      position,
      shape,
      transform,
      material,
    };
    let i2 = IntersectionRecord {
      t: 1.0,
      position,
      shape,
      transform,
      material,
    };
    assert_eq!(
      &i2,
      IntersectionSubsystem::nearest_hit(&vec![i1, i2]).unwrap()
    );

    // no intersection in frustum
    let i1 = IntersectionRecord {
      t: -2.0,
      position,
      shape,
      transform,
      material,
    };
    let i2 = IntersectionRecord {
      t: -1.0,
      position,
      shape,
      transform,
      material,
    };
    assert_eq!(None, IntersectionSubsystem::nearest_hit(&vec![i1, i2]));

    // smallest positive
    let i1 = IntersectionRecord {
      t: 5.0,
      position,
      shape,
      transform,
      material,
    };
    let i2 = IntersectionRecord {
      t: 7.0,
      position,
      shape,
      transform,
      material,
    };
    let i3 = IntersectionRecord {
      t: -3.0,
      position,
      shape,
      transform,
      material,
    };
    let i4 = IntersectionRecord {
      t: 2.0,
      position,
      shape,
      transform,
      material,
    };
    assert_eq!(
      &i4,
      IntersectionSubsystem::nearest_hit(&vec![i1, i2, i3, i4]).unwrap()
    );
  }
}