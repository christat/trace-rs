extern crate rusty_math as rm;

mod canvas;
mod color;
mod ray;
mod sphere;
mod intersection;

use canvas::Canvas;
use color::Color;
use rm::Tuple4;
use ray::Ray;
use sphere::{Sphere};
use intersection::Intersect;

fn main() {
    let canvas_pixels = 100;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let red = Color::new(1.0, 0.0, 0.0);

    let wall_z = 10.0;
    let wall_size = 7.0;
    let half_wall_size = wall_size / 2.0;
    let pixel_size = wall_size / canvas_pixels as f32;

    let sphere = Sphere::unit();
    let ray_origin = Tuple4::point(0.0, 0.0, -5.0);

    for x in 0..canvas_pixels {
        for y in 0 ..canvas_pixels {
            let p_x = -half_wall_size + (x as f32 * pixel_size);
            let p_y = half_wall_size - (y as f32 * pixel_size);
            let p = Tuple4::point(p_x, p_y, wall_z);
            let mut ray_direction = p - ray_origin;
            ray_direction.normalize();
            let ray = Ray::new(ray_origin, ray_direction);    
            
            if let Some(_) = sphere.intersects(ray) {
                canvas.write(x, y, &red);
            }
        }
    }
    canvas.export_ppm(&String::from("sphere.ppm"));
}
