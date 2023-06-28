use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct MovingSphere<Material> {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Material,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(cen0: Vec3, cen1: Vec3, _time0: f64, _time1: f64, r: f64, m: M) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time0: _time0,
            time1: _time1,
            radius: r,
            mat_ptr: m,
        }
    }
    fn center(&self, time: f64) -> Vec3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
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
        let outward_normal = (p - self.center(r.time)) / self.radius;
        let hit_rec = HitRecord::new(p, t, outward_normal, r, &self.mat_ptr);
        Some(hit_rec)
    }
}
