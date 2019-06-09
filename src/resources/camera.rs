use yaac::{Matrix4, Tuple4};

use crate::ray::Ray;

pub struct Camera {
  pub width: u16,
  pub height: u16,
  pub fov: f32,
  pub transform: Matrix4,
  pub half_width: f32,
  pub half_height: f32,
  pub pixel_size: f32,
}

impl Camera {
  pub fn new(width: u16, height: u16, fov: f32) -> Self {
    let half_view = f32::tan(fov / 2.0);
    let aspect_ratio = width as f32 / height as f32;
    let (half_width, half_height) = match aspect_ratio {
      aspect if aspect >= 1.0 => (half_view, half_view / aspect),
      aspect => (half_view * aspect, half_view),
    };
    let pixel_size = (half_width * 2.0) / width as f32;
    Self {
      width,
      height,
      fov,
      transform: Matrix4::identity(),
      half_width,
      half_height,
      pixel_size,
    }
  }

  pub fn ray_for_pixel(&self, x: u16, y: u16) -> Ray {
    let x_offset = (x as f32 + 0.5) * self.pixel_size;
    let y_offset = (y as f32 + 0.5) * self.pixel_size;
    let world_x = self.half_width - x_offset;
    let world_y = self.half_height - y_offset;

    let inverse_transform = self.transform.inverse().unwrap();
    let pixel = inverse_transform * Tuple4::point(world_x, world_y, -1.0);

    let origin = inverse_transform * Tuple4::point(0.0, 0.0, 0.0);
    let direction = (pixel - origin).normalized();
    Ray::new(origin, direction)
  }
}

impl Default for Camera {
  fn default() -> Self {
    let mut camera = Self::new(200, 200, std::f32::consts::PI / 4.0);
    camera.transform = view_transform(
      Tuple4::point(0.0, 0.0, -5.0),
      Tuple4::point(0.0, 0.0, 0.0),
      Tuple4::vector(0.0, 1.0, 0.0),
    );
    camera
  }
}

pub fn view_transform(from: Tuple4, to: Tuple4, up: Tuple4) -> Matrix4 {
  let forward = (to - from).normalized();
  let up_normalized = up.normalized();
  let left = Tuple4::cross(forward, up_normalized).unwrap();
  let true_up = Tuple4::cross(left, forward).unwrap();
  let orientation = Matrix4::new(
    Tuple4::new(left.x(), true_up.x(), -forward.x(), 0.0),
    Tuple4::new(left.y(), true_up.y(), -forward.y(), 0.0),
    Tuple4::new(left.z(), true_up.z(), -forward.z(), 0.0),
    Tuple4::new(0.0, 0.0, 0.0, 1.0),
  );
  orientation * Matrix4::translation(-from.x(), -from.y(), -from.z())
}

#[cfg(test)]
mod tests {
  mod camera {
    use super::super::Camera;
    use yaac::test_utils::cmp_tuple4;
    use yaac::{Matrix4, Tuple4};

    const HALF_PI: f32 = std::f32::consts::PI / 2.0;

    #[test]
    fn constructor() {
      let c = Camera::new(160, 120, HALF_PI);
      assert_eq!(160, c.width);
      assert_eq!(120, c.height);
      assert_eq!(HALF_PI, c.fov);
      assert_eq!(Matrix4::identity(), c.transform);
    }

    #[test]
    fn pixel_size() {
      let c = Camera::new(200, 125, HALF_PI);
      assert_eq!(0.01, c.pixel_size);

      let c = Camera::new(125, 200, HALF_PI);
      assert_eq!(0.01, c.pixel_size);
    }

    #[test]
    fn ray_for_pixel() {
      let mut c = Camera::new(201, 101, HALF_PI);
      let ray = c.ray_for_pixel(100, 50);
      assert!(cmp_tuple4(Tuple4::point(0.0, 0.0, 0.0), ray.origin));
      assert!(cmp_tuple4(Tuple4::vector(0.0, 0.0, -1.0), ray.direction));

      let ray = c.ray_for_pixel(0, 0);
      assert!(cmp_tuple4(Tuple4::point(0.0, 0.0, 0.0), ray.origin));
      assert!(cmp_tuple4(
        Tuple4::vector(0.66519, 0.33259, -0.66851),
        ray.direction
      ));

      c.transform =
        Matrix4::rotation_y(std::f32::consts::PI / 4.0) * Matrix4::translation(0.0, -2.0, 5.0);
      let half_sqrt_2 = f32::sqrt(2.0) / 2.0;
      let ray = c.ray_for_pixel(100, 50);
      assert!(cmp_tuple4(Tuple4::point(0.0, 2.0, -5.0), ray.origin));
      assert!(cmp_tuple4(
        Tuple4::vector(half_sqrt_2, 0.0, -half_sqrt_2),
        ray.direction
      ));
    }
  }

  mod functions {
    use super::super::view_transform;
    use yaac::test_utils::cmp_matrix4;
    use yaac::{Matrix4, Tuple4};

    #[test]
    fn view_transform_default() {
      let from = Tuple4::point(0.0, 0.0, 0.0);
      let to = Tuple4::point(0.0, 0.0, -1.0);
      let up = Tuple4::vector(0.0, 1.0, 0.0);
      assert_eq!(Matrix4::identity(), view_transform(from, to, up));
    }

    #[test]
    fn view_transform_looking_to_z() {
      let from = Tuple4::point(0.0, 0.0, 0.0);
      let to = Tuple4::point(0.0, 0.0, 1.0);
      let up = Tuple4::vector(0.0, 1.0, 0.0);
      assert_eq!(
        Matrix4::scaling(-1.0, 1.0, -1.0),
        view_transform(from, to, up)
      );
    }

    #[test]
    fn view_transform_moves_world() {
      let from = Tuple4::point(0.0, 0.0, 8.0);
      let to = Tuple4::point(0.0, 0.0, 0.0);
      let up = Tuple4::vector(0.0, 1.0, 0.0);
      assert!(cmp_matrix4(
        Matrix4::translation(0.0, 0.0, -8.0),
        view_transform(from, to, up)
      ));
    }

    #[test]
    fn view_transform_complex() {
      let from = Tuple4::point(1.0, 3.0, 2.0);
      let to = Tuple4::point(4.0, -2.0, 8.0);
      let up = Tuple4::vector(1.0, 1.0, 0.0);
      let expected = Matrix4::new(
        Tuple4::new(-0.50709, 0.76772, -0.35857, 0.0),
        Tuple4::new(0.50709, 0.60609, 0.59761, 0.0),
        Tuple4::new(0.67612, 0.12122, -0.71714, 0.0),
        Tuple4::new(-2.36643, -2.82843, 0.0, 1.0),
      );
      let result = view_transform(from, to, up);
      assert!(cmp_matrix4(expected, result));
    }
  }
}
