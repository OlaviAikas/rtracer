mod camera;
mod geometry;
mod light;
mod plane;
mod ray;
mod sphere;
mod typedefs;
mod vect;
use image::ImageFormat;
use std::f64::consts::PI;
use std::sync::Arc;
use typedefs::{Material, Scene};
use vect::*;

const EPSILON: f64 = 0.0001;
const IMAGE_WIDTH: u32 = 1000;
const IMAGE_HEIGTH: u32 = 1000;
const NRAYS: u32 = 100;
const DEPTH: u8 = 4;
const N_THREADS: usize = 8;

fn main() {
    let cam = camera::new(
        Vect(0f64, 2f64, 0f64), // Position
        Vect(0f64, 0f64, 1f64), // Direction
        Vect(0f64, 1f64, 0f64), // "Up" Direction
        0.8f64,                 // In-world distance from position to screen
        PI / 3f64,              // Viewing angle
    );
    let mut scene: Scene = (Vec::new(), Vec::new());
    //Center sphere
    scene.0.push(Box::new(sphere::Sphere {
        pos: Vect(0f64, 2f64, 10f64),
        radius: 2f64,
        material: Material::Lambertian,
        albedo: Vect(0.4, 0.25, 0.15),
    }));
    //Floor
    scene.0.push(Box::new(plane::Plane {
        point: Vect(0.0, 0f64, 0.0),
        normal: Vect(0f64, 1f64, 0f64).normalise(),
        albedo: Vect(0f64, 0f64, 1f64),
        material: Material::Lambertian,
    }));
    //Back wall
    scene.0.push(Box::new(plane::Plane {
        point: Vect(0.0, 0f64, 24f64),
        normal: Vect(0f64, 0f64, -1f64).normalise(),
        albedo: Vect(0f64, 1f64, 0f64),
        material: Material::Lambertian,
    }));
    //Ceiling
    scene.0.push(Box::new(plane::Plane {
        point: Vect(0.0, 12f64, 0f64),
        normal: Vect(0f64, -1f64, 0f64).normalise(),
        albedo: Vect(255f64, 0f64, 0f64),
        material: Material::Lambertian,
    }));
    //Front wall
    scene.0.push(Box::new(plane::Plane {
        point: Vect(0.0, 0f64, -12f64),
        normal: Vect(0f64, 0f64, 1f64).normalise(),
        albedo: Vect(255.0, 0f64, 255.0),
        material: Material::Lambertian,
    }));
    //Right wall
    scene.0.push(Box::new(plane::Plane {
        point: Vect(12.0, 0f64, 0f64),
        normal: Vect(-1f64, 0f64, 0f64).normalise(),
        albedo: Vect(255f64, 255f64, 0f64),
        material: Material::Lambertian,
    }));
    //Left wall
    scene.0.push(Box::new(plane::Plane {
        point: Vect(-12f64, 0f64, 20f64),
        normal: Vect(1f64, 0f64, 0f64).normalise(),
        albedo: Vect(0f64, 255f64, 255f64),
        material: Material::Lambertian,
    }));
    //Light
    scene.1.push(Box::new(light::Pointlight {
        pos: Vect(0.0, 5.0, 0.0),
        intensity: 1000000000f64,
    }));
    let scene_p = Arc::new(scene);
    let img = cam.render(scene_p, NRAYS, DEPTH);
    match img.save_with_format("test_img.png", ImageFormat::Png) {
        Ok(_) => println!("Yay, managed to save!"),
        Err(e) => println!("Oh fuck, {}", e),
    }
}
