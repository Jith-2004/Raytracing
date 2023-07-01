mod aabb;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod ray;
mod sphere;
mod texture;
mod vec3;

use camera::Camera;
use color::write_color;
use hittable::Hittable;
use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use material::Dielectric;
use material::Lambertian;
use material::Metal;
use moving_sphere::MovingSphere;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use std::fs::File;
use texture::{CheckerTexture, SolidColor, Texture};
pub use vec3::Vec3;

const AUTHOR: &str = "Stewie";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    if let Some(hit_record) = world.hit(*r, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = hit_record.mat_ptr.scatter(r, &hit_record) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Vec3::zero();
        }
    }

    let unit_direction = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
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

    let height: usize = 200;
    let width: usize = 300;
    let path = "output/2.3.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60
    let max_depth = 30;
    let aspect_ratio = 1.5;

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

    let mut world = HittableList::new();

    let checker_texture = CheckerTexture::new(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Box::new(checker_texture)),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.4, 0.2, 0.1)))),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
    )));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = SolidColor::new(Vec3::random() * Vec3::random());
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        Lambertian::new(Box::new(albedo)),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    world.add(Box::new(Sphere::new(center, 0.2, Metal::new(albedo, fuzz))));
                } else {
                    world.add(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
                }
            }
        }
    }

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
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
                pixel_color_ += ray_color(&r, &world, max_depth);
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
