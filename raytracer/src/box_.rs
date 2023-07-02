use crate::aabb::Aabb;
use crate::aarect::{XyRect, XzRect, YzRect};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Box_<Material> {
    pub sides: HittableList,
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub ptr: Material,
}

impl<M: Material + 'static> Box_<M> {
    pub fn new(p0: Vec3, p1: Vec3, mp0: M, mp1: M, mp2: M, mp3: M, mp4: M, mp5: M, mp6: M) -> Self {
        let box_min = p0;
        let box_max = p1;

        let mut sides = HittableList::new();

        sides.add(Box::new(XyRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            mp0,
        )));

        sides.add(Box::new(XyRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            mp1,
        )));

        sides.add(Box::new(XzRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            mp2,
        )));

        sides.add(Box::new(XzRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            mp3,
        )));

        sides.add(Box::new(YzRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            mp4,
        )));

        sides.add(Box::new(YzRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            mp5,
        )));

        Self {
            box_min,
            box_max,
            sides,
            ptr: mp6,
        }
    }
}

impl<M: Material> Hittable for Box_<M> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let output_box = Aabb::new(self.box_min, self.box_max);
        Some(output_box)
    }
}
