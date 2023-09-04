use std::fs;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec3;

use camera::Camera;
use color::Color;
use hittable::HitRecord;
use hittable::Hittable;
use hittable_list::HittableList;
use interval::Interval;
use ray::Ray;
use vec3::Point3;
use vec3::Vec3;

fn ray_color(r: Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(&r, Interval::new(0.0, utils::INF), &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let uni_direction = Vec3::unit(&r.direction());
    let beta = 0.5 * (uni_direction.y() + 1.0);

    Color::same(1.0) * (1.0 - beta) + Color::new(0.5, 0.7, 1.0) * beta
}

fn main() -> Result<(), std::io::Error> {
    // World
    let mut world = HittableList::new();

    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let cam = Camera::new(aspect_ratio, image_width);

    // Render
    cam.render(&world, "image.ppm")?;

    Ok(())
}
