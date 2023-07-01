use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }

    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> bool {
        let t0 = if (self.minimum.x() - r.origin.x()) / r.direction.x()
            < (self.maximum.x() - r.origin.x()) / r.direction.x()
        {
            (self.minimum.x() - r.origin.x()) / r.direction.x()
        } else {
            (self.maximum.x() - r.origin.x()) / r.direction.x()
        };
        let t1 = if (self.minimum.x() - r.origin.x()) / r.direction.x()
            > (self.maximum.x() - r.origin.x()) / r.direction.x()
        {
            (self.minimum.x() - r.origin.x()) / r.direction.x()
        } else {
            (self.maximum.x() - r.origin.x()) / r.direction.x()
        };
        let mut _t_min = if t_min < t0 { t_min } else { t0 };
        let mut _t_max = if t_max < t1 { t_max } else { t1 };
        if _t_max <= _t_min {
            return false;
        }
        let t0 = if (self.minimum.y() - r.origin.y()) / r.direction.y()
            < (self.maximum.y() - r.origin.y()) / r.direction.y()
        {
            (self.minimum.y() - r.origin.y()) / r.direction.y()
        } else {
            (self.maximum.y() - r.origin.y()) / r.direction.y()
        };
        let t1 = if (self.minimum.y() - r.origin.y()) / r.direction.y()
            > (self.maximum.y() - r.origin.y()) / r.direction.y()
        {
            (self.minimum.y() - r.origin.y()) / r.direction.y()
        } else {
            (self.maximum.y() - r.origin.y()) / r.direction.y()
        };
        let mut _t_min = if t_min < t0 { _t_min } else { t0 };
        let mut _t_max = if t_max < t1 { _t_max } else { t1 };
        if _t_max <= _t_min {
            return false;
        }
        let t0 = if (self.minimum.z() - r.origin.z()) / r.direction.z()
            < (self.maximum.z() - r.origin.z()) / r.direction.z()
        {
            (self.minimum.z() - r.origin.z()) / r.direction.z()
        } else {
            (self.maximum.z() - r.origin.z()) / r.direction.z()
        };
        let t1 = if (self.minimum.z() - r.origin.z()) / r.direction.z()
            > (self.maximum.z() - r.origin.z()) / r.direction.z()
        {
            (self.minimum.z() - r.origin.z()) / r.direction.z()
        } else {
            (self.maximum.z() - r.origin.z()) / r.direction.z()
        };
        let mut _t_min = if t_min < t0 { _t_min } else { t0 };
        let mut _t_max = if t_max < t1 { _t_max } else { t1 };
        if _t_max <= _t_min {
            return false;
        }
        return true;
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = Vec3::new(
            if box0.minimum.x() < box1.minimum.x() {
                box0.minimum.x()
            } else {
                box1.minimum.x()
            },
            if box0.minimum.y() < box1.minimum.y() {
                box0.minimum.y()
            } else {
                box1.minimum.y()
            },
            if box0.minimum.z() < box1.minimum.z() {
                box0.minimum.z()
            } else {
                box1.minimum.z()
            },
        );

        let big = Vec3::new(
            if box0.maximum.x() > box1.maximum.x() {
                box0.maximum.x()
            } else {
                box1.maximum.x()
            },
            if box0.maximum.y() > box1.maximum.y() {
                box0.maximum.y()
            } else {
                box1.maximum.y()
            },
            if box0.maximum.z() > box1.maximum.z() {
                box0.maximum.z()
            } else {
                box1.maximum.z()
            },
        );

        AABB::new(small, big)
    }
}
