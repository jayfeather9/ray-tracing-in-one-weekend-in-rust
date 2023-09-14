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

    // 3 different materials for the spheres

    // let material_ground = material::Lambertian::new(Color::new(0.8, 0.8, 0.0));
    // let material_center = material::Lambertian::new(Color::new(0.1, 0.2, 0.5));
    // let material_left = material::Dielectric::new(1.5);
    // let material_right = material::Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    // let sphere_ground = sphere::Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    // let sphere_center = sphere::Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center);
    // let sphere_left = sphere::Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    // let sphere_left2 = sphere::Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, material_left);
    // let sphere_right = sphere::Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right);

    // world.add(sphere_ground);
    // world.add(sphere_center);
    // world.add(sphere_left);
    // world.add(sphere_left2);
    // world.add(sphere_right);

    // 2 red & blue spheres

    // let R = (std::f64::consts::PI / 4.0).cos();
    // let material_left = material::Lambertian::new(Color::new(0.0, 0.0, 1.0));
    // let material_right = material::Lambertian::new(Color::new(1.0, 0.0, 0.0));

    // world.add(sphere::Sphere::new(
    //     Point3::new(-R, 0.0, -1.0),
    //     R,
    //     material_left,
    // ));
    // world.add(sphere::Sphere::new(
    //     Point3::new(R, 0.0, -1.0),
    //     R,
    //     material_right,
    // ));

    // random spheres

    let ground_material = material::Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(sphere::Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let RANDOM_RADIUS = 11;
    for i in -RANDOM_RADIUS..RANDOM_RADIUS {
        for j in -RANDOM_RADIUS..RANDOM_RADIUS {
            let choose_mat = utils::random_double();
            let center = Point3::new(
                i as f64 + 0.9 * utils::random_double(),
                0.2,
                j as f64 + 0.9 * utils::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    world.add(sphere::Sphere::new(
                        center,
                        0.2,
                        material::Lambertian::new(albedo),
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_in(0.5, 1.0);
                    let fuzz = utils::random_double_in(0.0, 0.5);
                    world.add(sphere::Sphere::new(
                        center,
                        0.2,
                        material::Metal::new(albedo, fuzz),
                    ));
                } else {
                    // glass
                    world.add(sphere::Sphere::new(
                        center,
                        0.2,
                        material::Dielectric::new(1.5),
                    ));
                };
            }
        }
    }

    let material1 = material::Dielectric::new(1.5);
    world.add(sphere::Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ));

    let material2 = material::Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(sphere::Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ));

    let material3 = material::Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(sphere::Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 10; // 500
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    // Render
    cam.render(&world, "image.ppm", 1)?;

    Ok(())
}
