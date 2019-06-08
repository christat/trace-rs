use specs::{Builder, Entity, World};

use crate::components::{Material, Position, Shape, Sphere as SphereShape, Transform};

pub struct Sphere;

impl Sphere {
  pub fn new(
    world: &mut World,
    position: Position,
    shape: SphereShape,
    transform: Transform,
    material: Material,
  ) -> Entity {
    world
      .create_entity()
      .with(position)
      .with(Shape::Sphere(shape))
      .with(transform)
      .with(material)
      .build()
  }

  /* pub fn unit(world: &mut World) -> Entity {
     world
      .create_entity()
      .with(Position::origin())
      .with(Shape::Sphere(SphereShape::default()))
      .with(Transform::default())
      .with(Material::default())
      .build()
  } */
}
