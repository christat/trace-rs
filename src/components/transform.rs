use rm::Matrix4;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Transform(pub rm::Matrix4);

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Transform {
    pub fn new(transform: Matrix4) -> Self {
        Self(transform)
    }

    pub fn default() -> Self {
        Self(Matrix4::identity())
    }
}

#[cfg(test)]
mod tests {
    use super::{Matrix4, Transform};
    use rm::Tuple4;

    #[test]
    fn new() {
        let mat = Matrix4::new(
            Tuple4::new(0.0, 1.0, 2.0, 3.0),
            Tuple4::new(4.0, 5.0, 6.0, 7.0),
            Tuple4::new(8.0, 9.0, 10.0, 11.0),
            Tuple4::new(12.0, 13.0, 14.0, 15.0)
        );
        let tr = Transform::new(mat);
        assert_eq!(mat, tr.0);
    }

    #[test]
    fn default() {
        let id = Transform::default();
        assert_eq!(Matrix4::identity(), id.0);
    }
}