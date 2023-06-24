use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(cen: Vec3, r: f64) -> Self {
        Self {
            center: cen,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.squared_length();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let root = discriminant.sqrt();
        let mut temp = (-half_b - root) / a;
        if temp < t_min || t_max < temp {
            temp = (-half_b + root) / a;
            if temp < t_min || t_max < temp {
                return None;
            }
        }

        let t = temp;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let hit_rec = HitRecord::new(p, t, outward_normal, r);
        Some(hit_rec)
    }
}
