use specs::{ReadStorage, System};

use crate::components::{Position};

struct Intersection;

impl<'a> System<'a> for Intersection {
  type SystemData = ReadStorage<'a, Position>;

  fn run(&mut self, positions: Self::SystemData) {
    use specs::Join;

    for position in positions.join() {
      println!("Hello, {:?}", &position);
    }
  }
}