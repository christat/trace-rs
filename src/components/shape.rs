use specs::{Component, VecStorage};

#[derive(Debug)]
pub enum Shape {
    Sphere(SphereShape)
}

impl Component for Shape {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Clone, Copy)]
pub struct SphereShape {
    pub radius: f32
}