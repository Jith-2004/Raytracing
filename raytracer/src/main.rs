mod aabb;
mod aarect;
mod box_;
mod camera;
mod color;
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
use hittable::Hittable;
use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use material::{Dielectric, DiffuseLight, Lambertian, Metal};
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
    let path = "output/2.15.jpg";
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

    let mut world = HittableList::new();

    world.add(Box::new(YzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.12, 0.45, 0.15)))),
    )));

    world.add(Box::new(YzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.65, 0.05, 0.05)))),
    )));

    world.add(Box::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73)))),
    )));

    world.add(Box::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73)))),
    )));

    world.add(Box::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73)))),
    )));

    world.add(Box::new(XzRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        DiffuseLight::new(Box::new(SolidColor::new(Vec3::new(15.0, 15.0, 15.0)))),
    )));
    let white0 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))));
    let white1 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))));
    let white2 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))));
    let white3 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))));
    let white4 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))));
    let white5 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))));
    let white6 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))));
    world.add(Box::new(Box_::new(
        Vec3::new(130.0, 0.0, 65.0),
        Vec3::new(295.0, 165.0, 230.0),
        white0,
        white1,
        white2,
        white3,
        white4,
        white5,
        white6,
    )));
    let white0 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))));
    let white1 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))));
    let white2 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))));
    let white3 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))));
    let white4 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))));
    let white5 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))));
    let white6 = Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73))));
    world.add(Box::new(Box_::new(
        Vec3::new(265.0, 0.0, 295.0),
        Vec3::new(430.0, 330.0, 460.0),
        white0,
        white1,
        white2,
        white3,
        white4,
        white5,
        white6,
    )));

    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
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

    let mut rng = rand::thread_rng();

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
