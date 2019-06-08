use specs::{Component, VecStorage};
use crate::components::color;

mod phong;

pub use phong::Phong;

#[derive(Debug, PartialEq)]
pub enum MaterialType {
    Phong(Phong)
}

#[derive(Debug, PartialEq)]
pub struct Material(pub MaterialType);

impl Default for Material {
    fn default() -> Self {
        Self(MaterialType::Phong(Phong::default()))
    }
}

impl Component for Material {
    type Storage = VecStorage<Self>;
}