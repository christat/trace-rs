use rm::Tuple3;
use specs::{Component, VecStorage};

#[derive(Debug, PartialEq)]
pub struct AABB {
    pub max: Tuple3,
    pub min: Tuple3
}

impl Component for AABB {
    type Storage = VecStorage<Self>;
}

impl AABB {
    pub fn new(max_x: f32, max_y: f32, max_z: f32, min_x: f32, min_y: f32, min_z: f32) -> Self {
        Self {
            max: Tuple3::new(max_x, max_y, max_z),
            min: Tuple3::new(min_x, min_y, min_z)
        }
    }
}

#[cfg(test)]
mod tests {
  use super::{AABB, Tuple3};

  #[test]
  fn constructor() {
      let aabb = AABB::new(5.0, 10.0, 15.0, -5.0, -10.0, -15.0);
      assert_eq!(AABB {
          max: Tuple3::new(5.0, 10.0, 15.0),
          min: Tuple3::new(-5.0, -10.0, -15.0)
      }, aabb);
  }
}