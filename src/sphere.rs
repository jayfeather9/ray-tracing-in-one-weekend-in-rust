use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Debug)]
pub struct Sphere<M: Material> {
    pub center: Point3,
    pub radius: f64,
    pub mat: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, mat: M) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot_square();
        let half_b = oc.dot(&r.direction());
        let c = oc.dot_square() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.contains(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.contains(root) {
                return None;
            }
        }

        let mut rec = HitRecord::new();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = &self.mat;
        Some(rec)
    }
}
