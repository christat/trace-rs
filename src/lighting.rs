extern crate rusty_math as rm;

use crate::color::{Color, BLACK};
use crate::material::Material;
use crate::point_light::PointLight;
use rm::Tuple4;


pub fn lighting(mat: &Material, light: &PointLight, point: Tuple4, eye_v: Tuple4, normal_v: Tuple4) -> Color {
    let color = mat.color * light.intensity;
    let ambient = color * mat.ambient;
    let mut diffuse = BLACK;
    let mut specular = BLACK;

    let mut light_v = light.position - point;
    light_v.normalize();
    let light_dot_normal = Tuple4::dot(light_v, normal_v);
    if light_dot_normal >= 0.0 {
      diffuse = color * mat.diffuse * light_dot_normal;
      let reflect_v = Tuple4::reflect(-light_v, normal_v);
      let reflect_dot_eye = Tuple4::dot(reflect_v, eye_v);
      if reflect_dot_eye > 0.0 {
        let factor = reflect_dot_eye.powf(mat.shininess);
        specular = light.intensity * mat.specular * factor;
      }
    }
    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
  use super::{Color, lighting, Material, PointLight, Tuple4};
  use rusty_math::test_utils;

  fn cmp_color(lhs: Color, rhs: Color) -> bool {
    test_utils::cmp_f32(lhs.r(), rhs.r()) &&
    test_utils::cmp_f32(lhs.g(), rhs.g()) &&
    test_utils::cmp_f32(lhs.b(), rhs.b())
  }

  #[test]
  fn test_lighting_case_eye_between_light_and_surface() {
    let m = Material::default();
    let pos = Tuple4::point(0.0, 0.0, 0.0);
    let eye_v = Tuple4::vector(0.0, 0.0, -1.0);
    let normal_v = Tuple4::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple4::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
    assert!(cmp_color(Color::new(1.9, 1.9, 1.9), lighting(&m, &light, pos, eye_v, normal_v)));
  }

  #[test]
  fn test_lighting_case_eye_offset_45_deg_between_light_and_surface() {
    let m = Material::default();
    let pos = Tuple4::point(0.0, 0.0, 0.0);
    let coord = f32::sqrt(2.0) / 2.0;
    let eye_v = Tuple4::vector(0.0, coord, -coord);
    let normal_v = Tuple4::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple4::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
    assert!(cmp_color(Color::new(1.0, 1.0, 1.0), lighting(&m, &light, pos, eye_v, normal_v)));
  }

  #[test]
  fn test_lighting_case_eye_opposite_offset_45_deg_between_light_and_surface() {
    let m = Material::default();
    let pos = Tuple4::point(0.0, 0.0, 0.0);
    let eye_v = Tuple4::vector(0.0, 0.0, -1.0);
    let normal_v = Tuple4::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple4::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    assert!(cmp_color(Color::new(0.7363, 0.7363, 0.7363), lighting(&m, &light, pos, eye_v, normal_v)));
  }

  #[test]
  fn test_lighting_case_eye_in_path_of_reflection_vector() {
    let m = Material::default();
    let pos = Tuple4::point(0.0, 0.0, 0.0);
    let coord = f32::sqrt(2.0) / 2.0;
    let eye_v = Tuple4::vector(0.0, -coord, -coord);
    let normal_v = Tuple4::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple4::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    assert!(cmp_color(Color::new(1.6363962, 1.6363962, 1.6363962), lighting(&m, &light, pos, eye_v, normal_v)));
  }

  #[test]
  fn test_lighting_case_eye_behind_surface() {
    let m = Material::default();
    let pos = Tuple4::point(0.0, 0.0, 0.0);
    let eye_v = Tuple4::vector(0.0, 0.0, -1.0);
    let normal_v = Tuple4::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple4::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
    assert!(cmp_color(Color::new(0.1, 0.1, 0.1), lighting(&m, &light, pos, eye_v, normal_v)));
  }
}