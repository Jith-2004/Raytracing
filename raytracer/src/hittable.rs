use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, t: f64, outward_normal: Vec3, r: Ray) -> Self {
        let front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };

        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
