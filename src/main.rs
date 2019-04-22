extern crate rusty_math as rm;

mod cannon;
mod canvas;
mod color;

use cannon::{Projectile, Environment};
use canvas::Canvas;
use color::Color;
use rm::Tuple;

fn main() {
    let canvas_width = 900;
    let canvas_height = 550;
    let mut canvas = Canvas::new(canvas_width, canvas_height);
    let projectile_color = Color::new(1.0, 1.0, 1.0);

    let position = Tuple::point(0.0, 1.0, 0.0);
    let mut velocity = Tuple::vector(1.0, 1.8, 0.0);
    velocity.normalize();
    velocity *= 11.25;
    let mut p = Projectile { position: position, velocity: velocity };

    let gravity = Tuple::vector(0.0, -0.1, 0.0);
    let wind = Tuple::vector(-0.01, 0.0, 0.0);
    let e = Environment { gravity: gravity, wind: wind };

    let mut steps = 0;
    println!("Step | Position (x, y):");
    while p.position.y() > 0.0 {
        let canvas_x = f32::min(canvas_width as f32, f32::max(0.0, p.position.x())).round() as usize;
        let canvas_y = f32::min(canvas_height as f32, f32::max(0.0, canvas_height as f32 - p.position.y())).round() as usize;
        canvas.write(canvas_x, canvas_y, &projectile_color);
        println!("{:^4} | ({}, {})", steps, p.position.x(), p.position.y());
        p = cannon::tick(&e, p);
        steps += 1;
    }
    println!("\nProjectile took {} steps to hit the ground.", steps);
    canvas.export_ppm(&String::from("canvas.ppm"));
}
