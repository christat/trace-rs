use specs::{Component, VecStorage};
use crate::components::color;

mod phong;

pub use phong::Phong;

pub enum MaterialType {
    Phong(Phong)
}

pub struct Material(MaterialType);

impl Component for Material {
    type Storage = VecStorage<Self>;
}

impl Material {
    pub fn new(material: MaterialType) -> Self {
        Self(material)
    }

    pub fn default() -> Self {
        Self(MaterialType::Phong(Phong::default()))
    }
}