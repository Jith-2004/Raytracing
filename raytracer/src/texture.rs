use crate::perlin::Perlin;
use crate::vec3::Vec3;
use image::{DynamicImage, GenericImage, GenericImageView, RgbImage};
use std::path::Path;

fn clamp(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x > 0.99 {
        0.99
    } else {
        x
    }
}

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct SolidColor {
    pub color_value: Vec3,
}

impl SolidColor {
    pub fn new(c: Vec3) -> Self {
        Self { color_value: c }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color_value
    }
}

pub struct CheckerTexture {
    pub odd: SolidColor,
    pub even: SolidColor,
}

impl CheckerTexture {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            odd: SolidColor::new(a),
            even: SolidColor::new(b),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(sc: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        (1.0 + f64::sin(self.scale * p.z() + 10.0 * self.noise.turb(p, 7))) * Vec3::one() * 0.5
    }
}

pub struct ImageTexture {
    width: u32,
    height: u32,
    img: DynamicImage,
}

impl ImageTexture {
    pub fn new(path: &Path) -> Self {
        let img = image::open(path).unwrap();
        let (width, height) = img.dimensions();
        Self { width, height, img }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let u = clamp(u);
        let v = clamp(v);
        let mut i = (u * self.width as f64) as u32;
        let mut j = (v * self.height as f64) as u32;
        if i >= self.width {
            let i = self.width - 1;
        }
        if j >= self.height {
            let j = self.height - 1;
        }
        let j = self.height - j - 1;
        let data = self.img.get_pixel(i, j);
        Vec3::new(
            1.0 / 255.0 * data[0] as f64,
            1.0 / 255.0 * data[1] as f64,
            1.0 / 255.0 * data[2] as f64,
        )
    }
}
