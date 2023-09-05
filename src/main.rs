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
use hittable_list::HittableList;
use interval::Interval;
use vec3::Point3;
use vec3::Vec3;

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
    let samples_per_pixel = 100;
    let max_depth = 50;
    let cam = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    // Render
    cam.render(&world, "image.ppm", 5)?;

    Ok(())
}
