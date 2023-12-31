use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;
use rand::Rng;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(color: Box<dyn Texture>) -> Self {
        Self { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered: Ray = Ray::new(hit_record.p, scatter_direction, r_in.time);
        let attenuation = self.albedo.value(hit_record.u, hit_record.v, &hit_record.p);
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(color: Vec3, f: f64) -> Self {
        Self {
            albedo: color,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let unit_ray_direction = Vec3::unit_vector(r_in.direction);
        let reflected = Vec3::reflect(unit_ray_direction, hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            r_in.time,
        );
        let attenuation = self.albedo;
        if Vec3::dot(scattered.direction, hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Self { ref_idx: ri }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let etai_over_etat = if hit_record.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = Vec3::unit_vector(r_in.direction);
        let cos_theta = if Vec3::dot(-unit_direction, hit_record.normal) < 1.0 {
            Vec3::dot(-unit_direction, hit_record.normal)
        } else {
            1.0
        };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(unit_direction, hit_record.normal);
            let scattered = Ray::new(hit_record.p, reflected, r_in.time);
            return Some((scattered, attenuation));
        }
        let reflect_prob = Vec3::schlick(cos_theta, etai_over_etat);
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..1.0) < reflect_prob {
            let reflected = Vec3::reflect(unit_direction, hit_record.normal);
            let scattered = Ray::new(hit_record.p, reflected, r_in.time);
            return Some((scattered, attenuation));
        }
        let refracted = Vec3::refract(unit_direction, hit_record.normal, etai_over_etat);
        let scattered = Ray::new(hit_record.p, refracted, r_in.time);
        Some((scattered, attenuation))
    }
}

pub struct DiffuseLight {
    emit: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Box<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    albedo: Box<dyn Texture>,
}

impl Isotropic {
    pub fn new(color: Box<dyn Texture>) -> Self {
        Self { albedo: color }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let scattered = Ray::new(hit_record.p, Vec3::random_in_unit_sphere(), r_in.time);
        let attenuation = self.albedo.value(hit_record.u, hit_record.v, &hit_record.p);
        Some((scattered, attenuation))
    }
}
