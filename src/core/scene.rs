use specs::World;

use crate::core::{Camera, PointLight};
use crate::components;
use crate::entities::{Sphere};

pub struct Scene<'a> {
  world: World,
  camera: &'a Camera,
  lights: &'a [PointLight]
}

impl<'a> Scene<'a> {
  pub fn new(camera: &'a Camera, lights: &'a [PointLight]) -> Self {
    Self {
      world: World::new(),
      camera: camera,
      lights: lights
    }
  }

  pub fn init_world(&mut self) {
    let mut_w = &mut self.world;

    mut_w.register::<components::AABB>();
    mut_w.register::<components::Material>();
    mut_w.register::<components::Position>();
    mut_w.register::<components::Shape>();
    mut_w.register::<components::Transform>();

    // TODO replace with scene builder from source file
    Sphere::unit(mut_w);

    // TODO create systems
    //let mut hello_world = HelloWorld;
    //hello_world.run_now(&world.res);
    mut_w.maintain();
  }
}