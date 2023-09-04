use std::fs;

mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec3;

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
    // String for image output
    let mut s = String::new();

    // Image size
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

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

    // Viewport size
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let pixel00_loc = viewport_upper_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

    // Render
    let mut percentage = 0;
    s.push_str(&format!("P3\n{image_width} {image_height}\n255\n"));
    for j in 0..image_height {
        // Log
        if j as f64 / image_height as f64 * 100.0 > percentage as f64 {
            if percentage % 5 == 0 {
                println!("{}% finished", percentage);
            }
            percentage += 1;
        }
        for i in 0..image_width {
            let pixel_center = pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r, &world);
            color::write_color(&mut s, pixel_color);
        }
    }
    // Log
    println!("100% finished");

    // Write
    fs::write("image.ppm", s)?;

    // Log
    println!("Image saved");

    Ok(())
}
