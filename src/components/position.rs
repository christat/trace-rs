use yaac::Tuple4;
use specs::{Component, VecStorage};

#[derive(Debug, PartialEq)]
pub struct Position(pub Tuple4);

impl Default for Position {
    fn default() -> Self {
        Self(Tuple4::point(0.0, 0.0, 0.0))
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}