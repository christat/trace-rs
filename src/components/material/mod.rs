use specs::{Component, VecStorage};
use crate::components::color;

mod phong;

pub use phong::Phong;

#[derive(Debug, PartialEq)]
pub enum Material {
    Phong(Phong)
}

impl Component for Material {
    type Storage = VecStorage<Self>;
}

impl Default for Material {
    fn default() -> Self {
        Material::Phong(Phong::default())
    }
}
