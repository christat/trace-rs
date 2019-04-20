mod cannon;

fn main() {
    let mut p = cannon::Projectile::new(0.0, 1.0, 0.0, 1.0, 1.0, 0.0);
    let e = cannon::Environment::new(0.0, -0.1, 0.0, -0.01, 0.0, 0.0);

    let mut steps = 0;
    println!("Step | Position (x, y):");
    while p.position.y() > 0.0 {
        println!("{:^4} | ({}, {})", steps, p.position.x(), p.position.y());
        p = cannon::tick(&e, p);
        steps += 1;
    }
    println!("\nProjectile took {} steps to hit the ground.", steps);
}
