extern crate rayon;
extern crate rusty_math as rm;
extern crate specs;

mod canvas;
mod components;
mod ecs;
mod entities;
mod ray;
mod resources;
mod systems;
mod traits;

pub use canvas::{Canvas, Batch, BATCH_SIZE};
pub use ray::Ray;

use ecs::ECS;

fn main() {
  ECS::demo().run();
}
