use rm::Tuple4;

use crate::Ray;
use crate::components::{Material, Position, Shape, Transform};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntersectionRecord<'a> {
  pub t: f32,
  pub position: &'a Position,
  pub shape: &'a Shape,
  pub transform: &'a Transform,
  pub material: &'a Material,
}

pub trait Intersection {
  fn intersect<'a>(
    &self,
    ray: &Ray,
    position: &'a Position,
    shape: &'a Shape,
    transform: &'a Transform,
    material: &'a Material,
  ) -> Option<Vec<IntersectionRecord<'a>>>;
}

pub trait Normal {
  fn normal_at(&self, point: &Tuple4, pos: &Position, tr: &Transform) -> Tuple4;
}