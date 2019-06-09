use yaac::Tuple4;
use specs::{Component, VecStorage};

#[derive(Debug, PartialEq)]
pub struct Position(pub Tuple4);

impl Position {
    pub fn origin() -> Self {
        Self(Tuple4::point(0.0, 0.0, 0.0))
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[cfg(test)]
mod tests {
    use yaac::Tuple4;
    use super::Position;

    #[test]
    fn origin() {
        let pos = Position::origin();
        assert_eq!(Tuple4::point(0.0, 0.0, 0.0), pos.0);
    }
}