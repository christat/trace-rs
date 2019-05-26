use rm::{Matrix4, Tuple4};
use specs::{Builder, Entity, World};
use crate::components::{Position, Material, MaterialType, Transform};

pub struct Sphere;

impl Sphere {
  pub fn new(world: &mut World, position: Tuple4, transform: Option<Matrix4>, material: Option<MaterialType>) -> Entity {
    world
      .create_entity()
      .with(Position::new(position))
      .with(Transform::new(transform))
      .with(Material::new(material))
      .build()
  }

  pub fn unit(world: &mut World) -> Entity {
    Self::new(world, Tuple4::point(0.0, 0.0, 0.0), None, None)
  }
}