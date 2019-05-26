use specs::{Component, VecStorage};
use crate::components::color;

mod phong;

pub use phong::Phong;

pub enum MaterialType {
    Phong(Phong)
}

pub struct Material(Option<MaterialType>);

impl Component for Material {
    type Storage = VecStorage<Self>;
}

impl Material {
    pub fn new(material: Option<MaterialType>) -> Self {
        Self(material)
    }
}