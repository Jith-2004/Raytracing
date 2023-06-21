mod color;
mod vec3;

use color::write_color;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::fs::File;
pub use vec3::Vec3;

const AUTHOR: &str = "Your name";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    let height: usize = 800;
    let width: usize = 800;
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

    for j in 0..height {
        for i in 0..width {
            let pixel_color = [
                (j as f32 / height as f32 * 255.).floor() as u8,
                ((i + height - j) as f32 / (height + width) as f32 * 255.).floor() as u8,
                (i as f32 / height as f32 * 255.).floor() as u8,
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
