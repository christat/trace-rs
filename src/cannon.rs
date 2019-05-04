extern crate rusty_math as rm;

pub struct Projectile {
  pub position: rm::Tuple4,
  pub velocity: rm::Tuple4
}

pub struct Environment {
  pub gravity: rm::Tuple4,
  pub wind: rm::Tuple4
}

pub fn tick(env: &Environment, proj: Projectile) -> Projectile {
  Projectile {
    position: proj.position + proj.velocity,
    velocity: proj.velocity + env.gravity + env.wind
  }
}