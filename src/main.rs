extern crate rusty_math as rm;
extern crate specs;

mod core;
mod components;
mod entities;
mod systems;

use rm::Tuple4;
use crate::components::Color;
use crate::core::{Camera, PointLight, Scene};

fn main() {
  let camera = Camera::new(Tuple4::point(0.0, 0.0, 0.0));
  let lights = vec![PointLight::new(Tuple4::point(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0))];
  Scene::new(&camera, &lights).init_world();
}
