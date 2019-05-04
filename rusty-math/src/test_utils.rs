#[cfg(test)]
pub fn cmp_f32(a: f32, b: f32) -> bool {
  if (a - b) < 0.00001 { true } else { false }
}
