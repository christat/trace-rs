use crate::ray::Ray;

use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntersectionRecord<'a, T: Intersect> {
  pub t: f32,
  pub o: &'a T,
}

pub fn hit<'a, T>(
  intersections: &'a[IntersectionRecord<'a, T>],
) -> Option<&'a IntersectionRecord<'a, T>>
where
  T: Intersect,
{
  if intersections.len() == 0 {
    return None;
  }
  intersections
    .iter()
    .filter(|ir| ir.t.is_sign_positive())
    .min_by(|ir1, ir2| ir1.t.partial_cmp(&ir2.t).unwrap_or(Ordering::Equal))
}

// TODO revisit Sized trait, as it may be too restrictive
// TODO consider wrapping trait in Result<>
pub trait Intersect where Self: std::marker::Sized {
  fn intersects(&self, r: Ray) -> Option<Vec<IntersectionRecord<Self>>>;
}
