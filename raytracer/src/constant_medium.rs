use crate::aabb::Aabb;
use crate::aarect::{XyRect, XzRect, YzRect};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;
use rand::Rng;

pub struct ConstantMedium {
    neg_inv_density: f64,
    boundary: Box<dyn Hittable>,
    phase_function: Box<dyn Material>,
}

impl ConstantMedium {
    pub fn new(d: f64, b: Box<dyn Hittable>, a: Box<dyn Material>) -> Self {
        Self {
            neg_inv_density: -1.0 / d,
            boundary: b,
            phase_function: a,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(r, -f64::INFINITY, f64::INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, f64::INFINITY) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }
                let ray_length = r.direction.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * r.direction.length();
                let mut rng = rand::thread_rng();
                let hit_distance = self.neg_inv_density * f64::log2(rng.gen::<f64>());
                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let t = rec1.t + hit_distance / ray_length;
                let p = r.at(t);
                let normal = Vec3::new(1.0, 0.0, 0.0);
                let hit_record =
                    HitRecord::new(p, t, rec1.u, rec1.v, normal, r, &(*self.phase_function));
                Some(hit_record)
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
