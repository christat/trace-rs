use rm::Matrix4;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Transform(Option<rm::Matrix4>);

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Transform {
    pub fn new(transform: Option<Matrix4>) -> Self {
        Self(transform)
    }
}

#[cfg(test)]
mod tests {
    use super::{Transform, Matrix4};

    #[test]
    fn constructor() {
        let tr = Transform::new(Some(Matrix4::identity()));
        assert_eq!(Matrix4::identity(), tr.0.unwrap());
    }
}