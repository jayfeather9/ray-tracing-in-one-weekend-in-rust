use crate::interval::Interval;
use crate::material::Material;
use crate::material::DEFAULT_MATERIAL;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord<'_> {
    pub fn new() -> Self {
        Self {
            p: Point3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat: &DEFAULT_MATERIAL,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // sets the hit record normal vector
        // NOTE: the parameter outward_normal is assumed to be a unit vector
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable: std::fmt::Debug {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
