use rm::Tuple4;
use specs::{Component, VecStorage};

#[derive(Debug, PartialEq)]
pub struct Position(pub Tuple4);

impl Position {
    pub fn new(position: Tuple4) -> Self {
        Self(position)
    }

    pub fn origin() -> Self {
        Self(Tuple4::point(0.0, 0.0, 0.0))
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[cfg(test)]
mod tests {
    use super::{Position, Tuple4};

    #[test]
    fn implements_constructor() {
        let pos = Position::new(Tuple4::point(1.0, 2.0, 3.0));
        assert_eq!(Tuple4::point(1.0, 2.0, 3.0), pos.0);

        let pos = Position::origin();
        assert_eq!(Tuple4::point(0.0, 0.0, 0.0), pos.0);
    }
}