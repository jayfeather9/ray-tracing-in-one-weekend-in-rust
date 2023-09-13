mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
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

    let material_ground = material::Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = material::Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left = material::Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = material::Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    let sphere_ground = sphere::Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let sphere_center = sphere::Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center);
    let sphere_left = sphere::Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let sphere_right = sphere::Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right);

    world.add(sphere_ground);
    world.add(sphere_center);
    world.add(sphere_left);
    world.add(sphere_right);

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
