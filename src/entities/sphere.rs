use specs::{Builder, Entity, World};

use crate::components::{Material, Position, Shape, Transform};

pub struct Sphere;

impl Sphere {
  pub fn new(
    world: &mut World,
    position: Position,
    radius: f32,
    transform: Transform,
    material: Material,
  ) -> Entity {
    world
      .create_entity()
      .with(position)
      .with(Shape::Sphere { radius })
      .with(transform)
      .with(material)
      .build()
  }

  pub fn unit(world: &mut World) -> Entity {
     world
      .create_entity()
      .with(Position::default())
      .with(Shape::Sphere { radius: 1.0 })
      .with(Transform::default())
      .with(Material::default())
      .build()
  }
}
