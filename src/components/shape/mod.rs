use specs::{Component, VecStorage};
use yaac::Tuple4;

use crate::components::{Material, Position, Transform};
use crate::systems::IntersectionRecord;
use crate::Ray;

mod sphere;

pub use sphere::{intersect as intersect_sphere, normal_at as normal_at_sphere};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Sphere { radius: f32 },
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Sphere { radius: 1.0 }
    }
}

impl Component for Shape {
    type Storage = VecStorage<Self>;
}

impl<'a> Shape {
    pub fn intersect(
        &self,
        ray: &Ray,
        position: &'a Position,
        shape: &'a Shape,
        transform: &'a Transform,
        material: &'a Material,
    ) -> Option<Vec<IntersectionRecord<'a>>> {
        match self {
            Shape::Sphere { radius } => {
                intersect_sphere(*radius, ray, position, shape, transform, material)
            }
        }
    }

    pub fn normal_at(&self, point: &Tuple4, pos: &Position, tr: &Transform) -> Tuple4 {
        match self {
            Shape::Sphere { radius: _ } => normal_at_sphere(point, pos, tr),
        }
    }
}
