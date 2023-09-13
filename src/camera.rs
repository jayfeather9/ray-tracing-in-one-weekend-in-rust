use std::fs;

use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::utils;
use crate::utils::degrees_to_radians;
use crate::Color;
use crate::HitRecord;
use crate::HittableList;
use crate::Interval;
use crate::Point3;
use crate::Vec3;

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: i32,       // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_depth: i32,         // Maximum number of ray bounces into scene

    pub vfov: f64,        // Vertical view angle (field of view)
    pub lookfrom: Point3, // Point camera is looking from
    pub lookat: Point3,   // Point camera is looking at
    pub vup: Vec3,        // Camera-relative "up" direction

    image_height: i32,   // Rendered image height
    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below

    // Camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let center = lookfrom;

        let focal_length = (lookfrom - lookat).length();
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left = center - (w * focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
        }
    }

    pub fn default() -> Self {
        Self::new(
            1.0,
            100,
            10,
            10,
            90.0,
            Point3::new(0.0, 0.0, -1.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
    }

    fn ray_color(r: &Ray, depth: i32, world: &HittableList) -> Color {
        if depth <= 0 {
            // If we've exceeded the ray bounce limit, no more light is gathered.
            return Color::zero();
        }
        if let Some(rec) = world.hit(&r, Interval::new(0.0001, utils::INF)) {
            let mut scattered: Ray = Ray::new(Point3::zero(), Vec3::zero());
            let mut attenuation: Color = Color::zero();
            return if rec.mat.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                attenuation * Self::ray_color(&scattered, depth - 1, world)
            } else {
                Color::zero()
            };
        }

        let uni_direction = Vec3::unit(&r.direction());
        let beta = 0.5 * (uni_direction.y() + 1.0);

        Color::same(1.0) * (1.0 - beta) + Color::new(0.5, 0.7, 1.0) * beta
    }

    fn pixel_sample_square(&self) -> Vec3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + utils::random_double();
        let py = -0.5 + utils::random_double();
        // px, py are in (-0.5, 0.5)
        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + self.pixel_delta_u * i as f64 + self.pixel_delta_v * j as f64;
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - self.center;

        Ray::new(ray_origin, ray_direction)
    }

    pub fn render(
        &self,
        world: &HittableList,
        image_path: &str,
        log_interval: i32,
    ) -> Result<(), std::io::Error> {
        let mut percentage = 0;
        let mut s = String::from(&format!(
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        ));
        for j in 0..self.image_height {
            // Log
            if j as f64 / self.image_height as f64 * 100.0 > percentage as f64 {
                if percentage % log_interval == 0 {
                    println!("{}% finished", percentage);
                }
                percentage += 1;
            }
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                crate::color::write_color(&mut s, pixel_color, self.samples_per_pixel);
            }
        }
        // Log
        println!("100% finished");

        // Write
        fs::write(image_path, s)?;

        // Log
        println!("Image saved");

        Ok(())
    }
}
