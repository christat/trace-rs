use specs::{Component, VecStorage};
use yaac::Matrix4;

#[derive(Debug, PartialEq)]
pub struct Transform(pub Matrix4);

impl Default for Transform {
    fn default() -> Self {
        Self(Matrix4::identity())
    }
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
