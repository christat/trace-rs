extern crate rusty_math as rm;

pub struct Projectile {
  pub position: rm::Tuple,
  pub velocity: rm::Tuple
}

impl Projectile {
  pub fn new(pos_x: f32, pos_y: f32, pos_z: f32, vel_x: f32, vel_y: f32, vel_z: f32) -> Self {
    Projectile {
      position: rm::Tuple::point(pos_x, pos_y, pos_z),
      velocity: rm::Tuple::vector(vel_x, vel_y, vel_z)
    }
  }
}

pub struct Environment {
  pub gravity: rm::Tuple,
  pub wind: rm::Tuple
}

impl Environment {
  pub fn new(g_x: f32, g_y: f32, g_z: f32, wind_x: f32, wind_y: f32, wind_z: f32) -> Self {
    Environment {
      gravity: rm::Tuple::vector(g_x, g_y, g_z),
      wind: rm::Tuple::vector(wind_x, wind_y, wind_z)
    }
  }
}

pub fn tick(env: &Environment, proj: Projectile) -> Projectile {
  Projectile {
    position: proj.position + proj.velocity,
    velocity: proj.velocity + env.gravity + env.wind
  }
}