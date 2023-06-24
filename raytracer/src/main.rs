mod color;
mod vec3;
mod ray;

use color::write_color;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::fs::File;
pub use vec3::Vec3;
use ray::Ray;

const AUTHOR: &str = "Stewie";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn ray_color(r: Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(r.direction);

    let t = 0.5 * (unit_direction.y() + 1.0);

    return (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0);
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    let height: usize = 100;
    let width: usize = 200;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60

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

    let origin = Vec3::zero();
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);

    for j in (0..=height - 1).rev() {
        for i in 0..width {
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;
            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let r = Ray::new(origin, direction);
            let pixel_color_ = ray_color(r);
            let pixel_color = [
                (pixel_color_.x() * 255.999) as u8,
                (pixel_color_.y() * 255.999) as u8,
                (pixel_color_.z() * 255.999) as u8,
            ];
            write_color(pixel_color, &mut img, i, height - j - 1);
            bar.inc(1);
        }
    }

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
