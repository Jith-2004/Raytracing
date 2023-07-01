use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Sphere<Material> {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Material,
}

impl<M: Material> Sphere<M> {
    pub fn new(cen: Vec3, r: f64, m: M) -> Self {
        Self {
            center: cen,
            radius: r,
            mat_ptr: m,
        }
    }

    pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;
        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        (u, v)
    }
}

impl<M: Material> Hittable for Sphere<M> {
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
        let (u, v) = Sphere::<M>::get_sphere_uv(&outward_normal);
        let hit_rec = HitRecord::new(p, t, u, v, outward_normal, r, &self.mat_ptr);
        Some(hit_rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let output_box = Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(output_box)
    }
}
