use specs::{Dispatcher, DispatcherBuilder, World};

use crate::components;
use crate::entities;
use crate::systems::{TilingSystem, TracingSystem};

pub struct ECS<'a, 'b>(Dispatcher<'a, 'b>, World);

impl<'a, 'b> ECS<'a, 'b> {
  pub fn demo() -> Self {
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(TilingSystem, "tiling", &[])
        .with(TracingSystem, "tracing", &["tiling"])
        .build();
    dispatcher.setup(&mut world.res);

    let mut scene = Self(dispatcher, world);

    (&mut scene).register_demo_entities();
    scene
  }

  pub fn run(&mut self) {
    let ECS (dispatcher, world) = self;
    dispatcher.dispatch(&mut world.res);
    world.maintain();
  }

  fn register_demo_entities(&mut self) {
    use components::{Color, Material, Position, Phong, Transform};
  
    let mut phong = Phong::default();
    phong.color = Color::new(1.0, 0.2, 1.0);
    
    entities::Sphere::new(
      &mut self.1,
      Position::default(),
      1.0,
      Transform::default(),
      Material::Phong(phong)
    );
  }
}
