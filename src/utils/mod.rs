use crate::components::{Material, Position, Shape, Transform};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntersectionRecord<'a> {
  pub t: f32,
  pub position: &'a Position,
  pub shape: &'a Shape,
  pub transform: &'a Transform,
  pub material: &'a Material,
}