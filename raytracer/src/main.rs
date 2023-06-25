mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::write_color;
use hittable::Hittable;
use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use std::fs::File;
pub use vec3::Vec3;

const AUTHOR: &str = "Stewie";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    if let Some(hit_record) = world.hit(*r, 0.0, f64::INFINITY) {
        let target: Vec3 = hit_record.p + hit_record.normal + Vec3::random_in_unit_sphere();
        let ray = Ray::new(hit_record.p, target - hit_record.p);
        return 0.5 * ray_color(&ray, world, depth - 1);
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

    let height: usize = 400;
    let width: usize = 800;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60
    let max_depth = 2;

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
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new();

    let mut rng = rand::thread_rng();

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
