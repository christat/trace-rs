use yaac::Tuple4;

use crate::components::{Material, Position, Shape, Transform};
use crate::Ray;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntersectionRecord<'a> {
  pub t: f32,
  pub position: &'a Position,
  pub shape: &'a Shape,
  pub transform: &'a Transform,
  pub material: &'a Material,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitMetadata<'a> {
  pub hit: &'a IntersectionRecord<'a>,
  pub hit_point: Tuple4,
  pub eye_vector: Tuple4,
  pub normal_vector: Tuple4,
  pub hit_inside: bool,
}

impl<'a> HitMetadata<'a> {
  pub fn from(ray: &Ray, hit: &'a IntersectionRecord) -> Self {
    let hit_point = ray.point_at(hit.t);
    let eye_vector = -ray.direction;
    let mut normal_vector = hit.shape.normal_at(&hit_point, hit.position, hit.transform);

    // invert normal if hit originates on the inside
    let hit_inside = if Tuple4::dot(normal_vector, eye_vector) < 0.0 {
      true
    } else {
      false
    };
    if hit_inside {
      normal_vector = -normal_vector;
    };

    Self {
      hit,
      hit_point,
      eye_vector,
      normal_vector,
      hit_inside,
    }
  }
}

#[cfg(test)]
mod tests {
  
  // TODO TEST HitMetadata::from()

}
