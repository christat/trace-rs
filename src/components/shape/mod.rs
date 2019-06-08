use specs::{Component, VecStorage};

mod sphere;

pub use sphere::Sphere;

#[derive(Debug, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
}

impl Component for Shape {
    type Storage = VecStorage<Self>;
}