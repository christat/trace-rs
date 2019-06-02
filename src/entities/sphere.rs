use crate::components::{Material, MaterialType, Position, Shape, SphereShape, Transform};
use rm::{Matrix4, Tuple4};
use specs::{Builder, Entity, World};

pub struct Sphere;

impl Sphere {
  pub fn new(
    world: &mut World,
    position: Tuple4,
    shape: SphereShape,
    transform: Matrix4,
    material: MaterialType,
  ) -> Entity {
    world
      .create_entity()
      .with(Position::new(position))
      .with(Shape::Sphere(shape))
      .with(Transform::new(transform))
      .with(Material::new(material))
      .build()
  }

  pub fn unit(world: &mut World) -> Entity {
     world
      .create_entity()
      .with(Position::new(Tuple4::point(0.0, 0.0, 0.0)))
      .with(Shape::Sphere(SphereShape { radius: 1.0 }))
      .with(Transform::default())
      .with(Material::default())
      .build()
  }
}
