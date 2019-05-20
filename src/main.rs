extern crate rusty_math as rm;

mod canvas;
mod color;
mod material;
mod lighting;
mod point_light;
mod ray;
mod sphere;
mod intersection;

use canvas::Canvas;
use color::Color;
use lighting::lighting;
use material::Material;
use point_light::PointLight;
use rm::Tuple4;
use ray::Ray;
use sphere::{Sphere};
use intersection::{Intersect, hit};

fn main() {
    let canvas_pixels = 100;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let wall_z = 10.0;
    let wall_size = 7.0;
    let half_wall_size = wall_size / 2.0;
    let pixel_size = wall_size / canvas_pixels as f32;

    let mut material = Material::default();
    material.color = Color::new(1.0, 0.2, 1.0);

    let mut sphere = Sphere::unit();
    sphere.set_material(material);

    let ray_origin = Tuple4::point(0.0, 0.0, -5.0);

    let light = PointLight::new(Tuple4::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    for x in 0..canvas_pixels {
        for y in 0 ..canvas_pixels {
            let p_x = -half_wall_size + (x as f32 * pixel_size);
            let p_y = half_wall_size - (y as f32 * pixel_size);
            let p = Tuple4::point(p_x, p_y, wall_z);
            let mut ray_direction = p - ray_origin;
            ray_direction.normalize();
            let ray = Ray::new(ray_origin, ray_direction);    
            
            if let Some(intersections) = sphere.intersects(ray) {
                let hit_record = hit(&intersections).unwrap();
                let hit_point = ray.point_at(hit_record.t);
                let normal = hit_record.o.normal_at(hit_point);
                let eye = -ray.direction;
                let color = lighting(&hit_record.o.get_material(), &light, hit_point, eye, normal);
                canvas.write(x, y, &color);
            }
        }
    }
    canvas.export_ppm(&String::from("sphere.ppm"));
}
