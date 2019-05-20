use crate::Matrix4;
use crate::Tuple4;

pub fn cmp_f32(a: f32, b: f32) -> bool {
  if (a - b) < 0.00001 { true } else { false }
}

pub fn cmp_tuple4(a: Tuple4, b: Tuple4) -> bool {
  cmp_f32(a.x(), b.x()) &&
  cmp_f32(a.y(), b.y()) &&
  cmp_f32(a.z(), b.z()) && 
  cmp_f32(a.w(), b.w())
}

pub fn cmp_matrix4(a: Matrix4, b: Matrix4) -> bool {
  cmp_tuple4(a.c0, b.c0) &&
  cmp_tuple4(a.c1, b.c1) &&
  cmp_tuple4(a.c2, b.c2) &&
  cmp_tuple4(a.c3, b.c3)
}