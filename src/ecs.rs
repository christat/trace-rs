use specs::{DispatcherBuilder, World};
use yaac::Matrix4;

use crate::components;
use crate::entities::Sphere;
use crate::resources::{Camera, Lights};
use crate::systems::{TilingSystem, TracingSystem};

pub struct ECS(World);

impl ECS {
  /* pub fn new(_filePath: String) -> Self {
    // TODO figure out file format, handle parsing, etc. :)
    let mut scene = Self(World::new());
    (&mut scene).register_components();
    scene
  } */

  pub fn demo() -> Self {
    let mut scene = Self(World::new());
    (&mut scene).register_components();
    (&mut scene).register_demo_resources();
    (&mut scene).register_demo_entities();
    scene
  }

  pub fn run(&mut self) {
    let world = &mut self.0;
    let mut dispatcher = DispatcherBuilder::new()
        .with(TilingSystem, "tiling", &[])
        .with(TracingSystem, "tracing", &["tiling"])
        .build();
    dispatcher.dispatch(&mut world.res);
    world.maintain();
  }

  // TODO - MANUAL REGISTERING MAY NOT BE NEEDED
  fn register_components(&mut self) {
    let world = &mut self.0;
    world.register::<components::Material>();
    world.register::<components::Position>();
    world.register::<components::Shape>();
    world.register::<components::Tile>();
    world.register::<components::Transform>();
  }

  // TODO - MANUAL REGISTERING MAY NOT BE NEEDED
  fn register_demo_resources(&mut self) {
    let world = &mut self.0;
    world.add_resource(Camera::default());
    world.add_resource(Lights::default());
  }

  fn register_demo_entities(&mut self) {
    use components::{Color, Material, MaterialType, Position, Phong, Sphere as SphereShape, Transform};

    Sphere::new(
      &mut self.0,
      Position::origin(),
      SphereShape::default(),
      Transform::default(),
      Material(MaterialType::Phong(Phong::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0))),
    );
    Sphere::new(
      &mut self.0,
      Position::origin(),
      SphereShape::default(),
      Transform(Matrix4::scaling(0.5, 0.5, 0.5)),
      Material::default()
    );
  }
}
