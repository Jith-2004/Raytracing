use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct XyRect<Material> {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mp: Material,
}

impl<M: Material> XyRect<M> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: M) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mp,
        }
    }
}

impl<M: Material> Hittable for XyRect<M> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.z()) / r.direction.z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin.x() + t * r.direction.x();
        let y = r.origin.y() + t * r.direction.y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let p = r.at(t);
        let hit_rec = HitRecord::new(p, t, u, v, outward_normal, r, &self.mp);
        Some(hit_rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let output_box = Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        );
        Some(output_box)
    }
}
