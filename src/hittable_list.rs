use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::DEFAULT_MATERIAL;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // let mut temp_rec = HitRecord {
        //     p: Point3::zero(),
        //     normal: Vec3::zero(),
        //     mat: &DEFAULT_MATERIAL,
        //     t: 0.0,
        //     front_face: false,
        // };
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }
}
