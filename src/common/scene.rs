use rm::{Matrix4, Tuple4};
use specs::{RunNow, World};

use crate::resources::{Camera, Lights, Tiles};
use crate::components;
use crate::entities::Sphere;
use crate::systems::Tiling;

pub struct Scene(World);

impl Scene {
  /* pub fn new(_filePath: String) -> Self {
    // TODO figure out file format, handle parsing, etc. :)
    let mut scene = Self(World::new());
    (&mut scene).register_components();
    scene
  } */

  pub fn demo() -> Self {
    let mut scene = Self(World::new());
    (&mut scene).register_components();
    (&mut scene).register_demo_entities();
    scene
  }

  pub fn run(&mut self) {
    let world = &mut self.0;
    let mut tiling = Tiling;
    tiling.run_now(&world.res);
    world.maintain();
  }

  // TODO - MANUAL REGISTERING MAY NOT BE NEEDED
  fn register_components(&mut self) {
    let world = &mut self.0;
    world.register::<components::AABB>();
    world.register::<components::Material>();
    world.register::<components::Position>();
    world.register::<components::Shape>();
    world.register::<components::Transform>();
  }

  // TODO - MANUAL REGISTERING MAY NOT BE NEEDED
  fn register_demo_resources(&mut self) {
    let world = &mut self.0;
    world.add_resource(Camera::default());
    world.add_resource(Lights::default());
    world.add_resource(Tiles::default());
  }

  fn register_demo_entities(&mut self) {
    use components::{Color, MaterialType, Phong, SphereShape};

    let origin = Tuple4::point(0.0, 0.0, 0.0);
    let unit_sphere_shape = SphereShape { radius: 1.0 };
    Sphere::new(
      &mut self.0,
      origin,
      unit_sphere_shape,
      Matrix4::identity(),
      MaterialType::Phong(Phong::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0)),
    );
    Sphere::new(
      &mut self.0,
      origin,
      unit_sphere_shape,
      Matrix4::scaling(0.5, 0.5, 0.5),
      MaterialType::Phong(Phong::default())
    );
  }
}
