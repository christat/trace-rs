extern crate rusty_math as rm;
extern crate specs;

mod common;
mod resources;

mod components;
mod entities;
mod systems;

use common::Scene;

fn main() {
  Scene::demo().run();
}
