use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::vec::Vec;

pub struct HittableList {
    pub hittable_list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            hittable_list: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.hittable_list.push(object);
    }

    pub fn clear(&mut self) {
        self.hittable_list.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closet_so_far = t_max;

        for obj in self.hittable_list.iter() {
            if let Some(temp_rec) = obj.hit(r, t_min, closet_so_far) {
                closet_so_far = temp_rec.t;
                hit_anything = Some(temp_rec);
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.hittable_list.is_empty() {
            return None;
        }
        let mut output_box: Option<Aabb> = None;
        let mut first_box: bool = true;
        for obj in self.hittable_list.iter() {
            if let Some(temp_box) = obj.bounding_box(time0, time1) {
                output_box = if first_box {
                    Some(temp_box)
                } else {
                    Some(Aabb::surrounding_box(output_box.unwrap(), temp_box))
                };
                first_box = false;
            } else {
                return None;
            }
        }
        output_box
    }
}
