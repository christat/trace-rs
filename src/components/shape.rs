use specs::{Component, VecStorage};

#[derive(Debug)]
pub enum Shape {
    Sphere { radius: f32 },
    Cylinder { radius: f32, size: f32 }
}

impl Component for Shape {
    type Storage = VecStorage<Self>;
}