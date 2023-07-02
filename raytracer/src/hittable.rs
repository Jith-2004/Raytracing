use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat_ptr: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Vec3,
        t: f64,
        u: f64,
        v: f64,
        outward_normal: Vec3,
        r: Ray,
        mat_ptr: &'a dyn Material,
    ) -> Self {
        let front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            t,
            u,
            v,
            front_face,
            mat_ptr,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

pub struct Translate {
    pub offset: Vec3,
    pub ptr: Box<dyn Hittable>,
}

impl Translate {
    pub fn new(offset: Vec3, ptr: Box<dyn Hittable>) -> Self {
        Self { offset, ptr }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        if let Some(rec) = self.ptr.hit(moved_r, t_min, t_max) {
            let hit_record = HitRecord::new(
                rec.p + self.offset,
                rec.t,
                rec.u,
                rec.v,
                rec.normal,
                moved_r,
                rec.mat_ptr,
            );
            Some(hit_record)
        } else {
            return None;
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if let Some(output_box) = self.ptr.bounding_box(time0, time1) {
            let output_box = Aabb::new(
                output_box.minimum + self.offset,
                output_box.maximum + self.offset,
            );
            Some(output_box)
        } else {
            return None;
        }
    }
}

pub struct RotateY {
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: Aabb,
    pub ptr: Box<dyn Hittable>,
}

impl RotateY {
    pub fn new(angle: f64, ptr: Box<dyn Hittable>) -> Self {
        let radians = angle * std::f64::consts::PI / 180.0;
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);
        let Some(bbox) = ptr.bounding_box(0.0, 1.0) else { panic!("Wrong!") };
        let mut min = [f64::INFINITY; 3];
        let mut max = [-f64::INFINITY; 3];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.maximum.x() + (1 - i) as f64 * bbox.minimum.x();
                    let y = j as f64 * bbox.maximum.y() + (1 - j) as f64 * bbox.minimum.y();
                    let z = k as f64 * bbox.maximum.z() + (1 - k) as f64 * bbox.minimum.z();
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);
                    min[0] = f64::min(min[0], tester.x());
                    max[0] = f64::max(max[0], tester.x());
                    min[1] = f64::min(min[1], tester.y());
                    max[1] = f64::max(max[1], tester.y());
                    min[2] = f64::min(min[2], tester.z());
                    max[2] = f64::max(max[2], tester.z());
                }
            }
        }
        let bbox = Aabb::new(
            Vec3::new(min[0], min[1], min[2]),
            Vec3::new(max[0], max[1], max[2]),
        );
        Self {
            sin_theta,
            cos_theta,
            bbox,
            ptr,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin = Vec3::new(
            self.cos_theta * r.origin.x() - self.sin_theta * r.origin.z(),
            r.origin.y(),
            self.sin_theta * r.origin.x() + self.cos_theta * r.origin.z(),
        );
        let direction = Vec3::new(
            self.cos_theta * r.direction.x() - self.sin_theta * r.direction.z(),
            r.direction.y(),
            self.sin_theta * r.direction.x() + self.cos_theta * r.direction.z(),
        );
        let rotated_r = Ray::new(origin, direction, r.time);
        if let Some(rec) = self.ptr.hit(rotated_r, t_min, t_max) {
            let p = Vec3::new(
                self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z(),
                rec.p.y(),
                -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z(),
            );
            let normal = Vec3::new(
                self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z(),
                rec.normal.y(),
                -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z(),
            );
            let hit_record = HitRecord::new(p, rec.t, rec.u, rec.v, normal, r, rec.mat_ptr);
            Some(hit_record)
        } else {
            return None;
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let output_box = Aabb::new(self.bbox.minimum, self.bbox.maximum);
        Some(output_box)
    }
}
