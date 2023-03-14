mod camera;
mod geometry;
mod light;
mod plane;
mod ray;
mod scene_loader;
mod sphere;
mod typedefs;
mod vect;
use image::ImageFormat;
use scene_loader::load_scene;
use std::f64::consts::PI;
use std::sync::Arc;
use vect::*;

//Benchmark 10 rays 4 depth 8 threads
// 14899ms

//Benchmark 1000 rays 8 depth 8 threads
// 422566ms
const EPSILON: f64 = 0.0001;
const IMAGE_WIDTH: u32 = 1000;
const IMAGE_HEIGTH: u32 = 1000;
const NRAYS: u32 = 100;
const DEPTH: u8 = 8;
const N_THREADS: usize = 8;

fn main() {
    let cam = camera::new(
        Vect(0f64, 2f64, 0f64), // Position
        Vect(0f64, 0f64, 1f64), // Direction
        Vect(0f64, 1f64, 0f64), // "Up" Direction
        PI / 3f64,              // Viewing angle
    );
    let scene = match load_scene("scene.toml") {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let scene_p = Arc::new(scene);
    let img = cam.render(scene_p, NRAYS, DEPTH);
    match img.save_with_format("test_img.png", ImageFormat::Png) {
        Ok(_) => println!("Yay, managed to save!"),
        Err(e) => println!("Oh no!, {}", e),
    }
}
