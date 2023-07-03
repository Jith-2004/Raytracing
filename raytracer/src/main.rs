mod aabb;
mod aarect;
mod box_;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod sphere;
mod texture;
mod vec3;

use aarect::{XyRect, XzRect, YzRect};
use box_::Box_;
use camera::Camera;
use color::write_color;
use constant_medium::ConstantMedium;
use hittable::{Hittable, RotateY, Translate};
use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use material::{Dielectric, DiffuseLight, Isotropic, Lambertian, Metal};
use moving_sphere::MovingSphere;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use std::fs::File;
use std::path::Path;
use texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};
pub use vec3::Vec3;

const AUTHOR: &str = "Stewie";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn ray_color(r: &Ray, background: &Vec3, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    if let Some(hit_record) = world.hit(*r, 0.001, f64::INFINITY) {
        let emitted = hit_record
            .mat_ptr
            .emitted(hit_record.u, hit_record.v, &hit_record.p);
        if let Some((scattered, attenuation)) = hit_record.mat_ptr.scatter(r, &hit_record) {
            emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
        } else {
            return emitted;
        }
    } else {
        return *background;
    }
}

fn clamp(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x > 0.99 {
        0.99
    } else {
        x
    }
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    let height: usize = 600;
    let width: usize = 600;
    let path = "output/2.18.jpg";
    let quality = 100; // From 0 to 100, suggested value: 60
    let max_depth = 50;
    let aspect_ratio = 1.0;

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    // Progress bar UI powered by library `indicatif`
    // You can use indicatif::ProgressStyle to make it more beautiful
    // You can also use indicatif::MultiProgress in multi-threading to show progress of each thread
    let bar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    let background = Vec3::zero();

    let mut rng = rand::thread_rng();

    let mut world = HittableList::new();

    for i in 0..20 {
        for j in 0..20 {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;
            world.add(Box::new(Box_::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.48, 0.83, 0.53)))),
                Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.48, 0.83, 0.53)))),
                Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.48, 0.83, 0.53)))),
                Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.48, 0.83, 0.53)))),
                Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.48, 0.83, 0.53)))),
                Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.48, 0.83, 0.53)))),
                Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.48, 0.83, 0.53)))),
            )));
        }
    }

    let light = DiffuseLight::new(Box::new(SolidColor::new(Vec3::new(7.0, 7.0, 7.0))));
    world.add(Box::new(XzRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material =
        Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.7, 0.3, 0.1))));
    world.add(Box::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0),
    )));

    let boundary = Sphere::new(Vec3::new(360.0, 150.0, 45.0), 70.0, Dielectric::new(1.5));
    world.add(Box::new(boundary));

    let boundary = Sphere::new(Vec3::new(360.0, 150.0, 45.0), 70.0, Dielectric::new(1.5));
    let t = SolidColor::new(Vec3::new(0.2, 0.4, 0.9));
    let m = Isotropic::new(Box::new(t));
    world.add(Box::new(ConstantMedium::new(
        0.2,
        Box::new(boundary),
        Box::new(m),
    )));

    let boundary = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    let t = SolidColor::new(Vec3::new(1.0, 1.0, 1.0));
    let m = Isotropic::new(Box::new(t));
    world.add(Box::new(ConstantMedium::new(
        0.00001,
        Box::new(boundary),
        Box::new(m),
    )));

    let earth_texture = ImageTexture::new(&Path::new("earth.jpg"));
    world.add(Box::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        Lambertian::new(Box::new(earth_texture)),
    )));

    let pertext = NoiseTexture::new(0.1);
    world.add(Box::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new(Box::new(pertext)),
    )));

    for i in 0..1000 {
        world.add(Box::new(Sphere::new(
            Vec3::random_(0.0, 165.0) + Vec3::new(-100.0, 270.0, 395.0),
            10.0,
            Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73)))),
        )))
    }

    let lookfrom = Vec3::new(478.0, 278.0, -600.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    for j in (0..=height - 1).rev() {
        for i in 0..width {
            let mut pixel_color: [u8; 3] = [0, 0, 0];
            let mut pixel_color_ = Vec3::zero();
            for _s in 0..quality {
                let u = (i as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color_ += ray_color(&r, &background, &world, max_depth);
            }
            pixel_color[0] += (clamp((pixel_color_.x() / quality as f64).sqrt()) * 255.999) as u8;
            pixel_color[1] += (clamp((pixel_color_.y() / quality as f64).sqrt()) * 255.999) as u8;
            pixel_color[2] += (clamp((pixel_color_.z() / quality as f64).sqrt()) * 255.999) as u8;
            write_color(pixel_color, &mut img, i, height - j - 1);
            bar.inc(1);
        }
    }

    world.clear();

    // Finish progress bar
    bar.finish();

    // Output image to file
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
