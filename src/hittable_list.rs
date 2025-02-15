use std::sync::Arc;

use crate::color::Color;
use crate::geometry::{Interval, Ray};
use crate::hittable::{HitRecord, Hittable};
use crate::material::Lambertian;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut rec = HitRecord::new(Arc::new(Lambertian {
            albedo: Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }));
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            match object.hit(
                r,
                &Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
            ) {
                Some(hit_record) => {
                    hit_anything = true;
                    closest_so_far = hit_record.t;
                    rec = hit_record;
                }
                None => continue,
            }
        }

        if hit_anything {
            Some(rec)
        } else {
            None
        }
    }
}
