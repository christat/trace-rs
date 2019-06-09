use yaac::Tuple4;

use crate::components::{BLACK, Color, Material, MaterialType, Phong};
use crate::resources::PointLight;

pub struct LightingSubsystem;

impl LightingSubsystem {
  pub fn run<'a>(material: &Material, light: &PointLight, point: &Tuple4, eye_v: &Tuple4, normal_v: &Tuple4) -> Color {
    match material.0 {
      MaterialType::Phong(m) => Self::phong(&m, light, point, eye_v, normal_v)
    }
  }

  pub fn phong(phong: &Phong, light: &PointLight, point: &Tuple4, eye_v: &Tuple4, normal_v: &Tuple4) -> Color {
    let color = phong.color * light.intensity;
    let ambient = color * phong.ambient;
    let mut diffuse = BLACK;
    let mut specular = BLACK;

    let mut light_v = light.position - *point;
    light_v.normalize();
    let light_dot_normal = Tuple4::dot(light_v, *normal_v);
    if light_dot_normal >= 0.0 {
      diffuse = color * phong.diffuse * light_dot_normal;
      let reflect_v = Tuple4::reflect(-light_v, *normal_v);
      let reflect_dot_eye = Tuple4::dot(reflect_v, *eye_v);
      if reflect_dot_eye > 0.0 {
        let factor = reflect_dot_eye.powf(phong.shininess);
        specular = light.intensity * phong.specular * factor;
      }
    }
    ambient + diffuse + specular
  }
}

#[cfg(test)]
mod tests {
  use super::{Color, LightingSubsystem, Phong, PointLight, Tuple4};
  use yaac::test_utils;

  fn cmp_color(lhs: Color, rhs: Color) -> bool {
    test_utils::cmp_f32(lhs.r(), rhs.r()) &&
    test_utils::cmp_f32(lhs.g(), rhs.g()) &&
    test_utils::cmp_f32(lhs.b(), rhs.b())
  }

  #[test]
  fn eye_between_light_and_surface() {
    let m = Phong::default();
    let pos = Tuple4::point(0.0, 0.0, 0.0);
    let eye_v = Tuple4::vector(0.0, 0.0, -1.0);
    let normal_v = Tuple4::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple4::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
    assert!(cmp_color(Color::new(1.9, 1.9, 1.9), LightingSubsystem::phong(&m, &light, &pos, &eye_v, &normal_v)));
  }

  #[test]
  fn eye_45_deg_between_light_and_surface() {
    let m = Phong::default();
    let pos = Tuple4::point(0.0, 0.0, 0.0);
    let coord = f32::sqrt(2.0) / 2.0;
    let eye_v = Tuple4::vector(0.0, coord, -coord);
    let normal_v = Tuple4::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple4::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
    assert!(cmp_color(Color::new(1.0, 1.0, 1.0), LightingSubsystem::phong(&m, &light, &pos, &eye_v, &normal_v)));
  }

  #[test]
  fn eye_neg_45_deg_between_light_and_surface() {
    let m = Phong::default();
    let pos = Tuple4::point(0.0, 0.0, 0.0);
    let eye_v = Tuple4::vector(0.0, 0.0, -1.0);
    let normal_v = Tuple4::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple4::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    assert!(cmp_color(Color::new(0.7363, 0.7363, 0.7363), LightingSubsystem::phong(&m, &light, &pos, &eye_v, &normal_v)));
  }

  #[test]
  fn eye_in_reflection_vector_path() {
    let m = Phong::default();
    let pos = Tuple4::point(0.0, 0.0, 0.0);
    let coord = f32::sqrt(2.0) / 2.0;
    let eye_v = Tuple4::vector(0.0, -coord, -coord);
    let normal_v = Tuple4::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple4::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    assert!(cmp_color(Color::new(1.6363962, 1.6363962, 1.6363962), LightingSubsystem::phong(&m, &light, &pos, &eye_v, &normal_v)));
  }

  #[test]
  fn eye_behind_surface() {
    let m = Phong::default();
    let pos = Tuple4::point(0.0, 0.0, 0.0);
    let eye_v = Tuple4::vector(0.0, 0.0, -1.0);
    let normal_v = Tuple4::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple4::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
    assert!(cmp_color(Color::new(0.1, 0.1, 0.1), LightingSubsystem::phong(&m, &light, &pos, &eye_v, &normal_v)));
  }
}