extern crate rusty_math as rm;

pub struct Projectile {
  pub position: rm::Tuple,
  pub velocity: rm::Tuple
}

pub struct Environment {
  pub gravity: rm::Tuple,
  pub wind: rm::Tuple
}

pub fn tick(env: &Environment, proj: Projectile) -> Projectile {
  Projectile {
    position: proj.position + proj.velocity,
    velocity: proj.velocity + env.gravity + env.wind
  }
}