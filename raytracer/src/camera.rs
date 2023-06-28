use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
        _time0: f64,
        _time1: f64,
    ) -> Self {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = f64::tan(theta / 2.0);
        let half_width = aspect * half_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::cross(vup, w);
        let v = Vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = 2.0 * half_width * u * focus_dist;
        let vertical = 2.0 * half_height * v * focus_dist;
        let lower_left_corner =
            origin - half_width * u * focus_dist - half_height * v * focus_dist - w * focus_dist;

        let lens_radius = aperture / 2.0;
        let time0 = _time0;
        let time1 = _time1;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = u * rd.x() + v * rd.y();
        let direction =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset;
        let mut rng = rand::thread_rng();
        Ray::new(
            self.origin + offset,
            direction,
            rng.gen_range(self.time0..self.time1),
        )
    }
}
