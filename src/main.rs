extern crate rusty_math as rm;

mod canvas;
mod color;

use canvas::Canvas;
use color::Color;
use rm::{Matrix4, Tuple4};

fn main() {
    let canvas_width = 150;
    let canvas_height = 100;
    let mut canvas = Canvas::new(canvas_width, canvas_height);
    let white = Color::new(1.0, 1.0, 1.0);

    let center_x = 0.5 * canvas_width as f32;
    let center_y = 0.5 * canvas_height as f32;
    let radius = 2.0/5.0 * usize::min(canvas_width, canvas_height) as f32;
    let clock_center = Tuple4::point(0.0, 0.0, 0.0);

    for i in 0..12 {
        let transform = Matrix4::translation(0.0, radius, 0.0)
            .rotate_z(i as f32 * std::f32::consts::PI/6.0)
            .translate(center_x, center_y, 0.0);
        let p = transform * clock_center;
        canvas.write(p.x() as usize, p.y() as usize, &white);
    }
    canvas.export_ppm(&String::from("clock.ppm"));
}
