extern crate rayon;
extern crate specs;
extern crate yaac;

mod components;
mod entities;
mod resources;
mod systems;

mod canvas;
mod ecs;
mod ray;
mod utils;

pub use canvas::{Canvas, Batch, BATCH_SIZE};
pub use ray::Ray;

use ecs::ECS;

fn main() {
  ECS::demo().run();
}
