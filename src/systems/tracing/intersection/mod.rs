use specs::{Join, Read, ReadStorage};
use std::cmp::Ordering;

use crate::components::{Color, Material, Position, Shape, Transform};
use crate::resources::Lights;
use crate::systems::LightingSubsystem;
use crate::Ray;

pub mod utils;

pub use utils::{HitMetadata, IntersectionRecord};

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
    
    Self::sort_hits(&mut intersection_records);

    let hit_record = Self::nearest_hit(&intersection_records);
    if hit_record.is_none() {
      return None;
    }

    let hit = hit_record.unwrap();
    let hit_metadata = HitMetadata::from(ray, hit);
    Some(LightingSubsystem::run(
      hit.material,
      &lights.0,
      &hit_metadata.hit_point,
      &hit_metadata.eye_vector,
      &hit_metadata.normal_vector,
    ))
  }

  fn sort_hits<'a>(records: &'a mut [IntersectionRecord]) {
    records.sort_by(|ir1, ir2| ir1.t.partial_cmp(&ir2.t).unwrap_or(Ordering::Equal));
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
  
  // TODO TEST SYSTEM, SORT_HITS AND NEAREST_HIT

}
