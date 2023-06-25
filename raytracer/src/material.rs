use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Self {
        Self { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered: Ray = Ray::new(hit_record.p, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(color: Vec3) -> Self {
        Self { albedo: color }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let unit_ray_direction = Vec3::unit_vector(r_in.direction);
        let reflected = Vec3::reflect(unit_ray_direction, hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected);
        let attenuation = self.albedo;
        if Vec3::dot(scattered.direction, hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
