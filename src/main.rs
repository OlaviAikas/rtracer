mod vect3;
use image::{ImageFormat, Rgb, RgbImage};

const EPSILON: f64 = 0.0000000001;
const IMAGE_WIDTH: u32 = 1000;
const IMAGE_HEIGHT: u32 = 1000;

fn main() {
    let mut img = RgbImage::new(IMAGE_HEIGHT, IMAGE_HEIGHT);
    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            if (x + y) % 2 == 0 {
                img.put_pixel(x, y, Rgb([0, 0, 0]));
            } else {
                img.put_pixel(x, y, Rgb([255, 0, 0]));
            }
        }
    }
    match img.save_with_format("test_img.png", ImageFormat::Png) {
        Ok(_) => println!("Yay, managed to save!"),
        Err(e) => println!("Oh fuck, {}", e),
    }
}
