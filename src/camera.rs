use std::fs;

use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::utils;
use crate::Color;
use crate::HitRecord;
use crate::Interval;
use crate::Point3;
use crate::Vec3;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left = Point3::new(0.0, 0.0, 0.0)
            - viewport_u / 2.0
            - viewport_v / 2.0
            - Vec3::new(0.0, 0.0, focal_length);
        let pixel00_loc = viewport_upper_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;
        Self {
            aspect_ratio,
            image_width,
            image_height,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::new();
        if world.hit(&r, Interval::new(0.0, utils::INF), &mut rec) {
            return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let uni_direction = Vec3::unit(&r.direction());
        let beta = 0.5 * (uni_direction.y() + 1.0);

        Color::same(1.0) * (1.0 - beta) + Color::new(0.5, 0.7, 1.0) * beta
    }

    pub fn render(&self, world: &dyn Hittable, image_path: &str) -> Result<(), std::io::Error> {
        let mut percentage = 0;
        let mut s = String::from(&format!(
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        ));
        for j in 0..self.image_height {
            // Log
            if j as f64 / self.image_height as f64 * 100.0 > percentage as f64 {
                if percentage % 5 == 0 {
                    println!("{}% finished", percentage);
                }
                percentage += 1;
            }
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + self.pixel_delta_u * i as f64
                    + self.pixel_delta_v * j as f64;
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = Self::ray_color(&r, world);
                crate::color::write_color(&mut s, pixel_color);
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
