use crate::ray::*;
use crate::typedefs::Scene;
use crate::vect::*;
use image::{Rgb, RgbImage};
use std::cmp::min;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Instant;

const GAMMA: f64 = 0.45;

/// The Camera type is a product type that contains the position of the
/// camera, the direction its facing, the direction that's "up" from the
/// camera's point of view, the distance from the camera to the screen and the
/// viewing angle of the camera, which determines the screen size in the scene
/// together with the focal distance.
///               pos   dir   up    dist angle
pub struct Camera(Vect, Vect, Vect, f64, f64);

/// Create a new camera with the given position, direction,
/// up direction and angle. Enforces that dot(dir, up) == 0,
/// and norms pos dir and up just to be safe.
pub fn new(pos: Vect, dir: Vect, up: Vect, dist: f64, angle: f64) -> Camera {
    if dir.dot(&up) != 0f64 {
        panic!(
            "Tried to create camera with non-perpendicular
        facing and up directions"
        );
    }
    Camera(pos, dir.normalise(), up.normalise(), dist, angle)
}

impl Camera {
    pub fn rays(&self) -> RayIter {
        let Camera(pos, dir, up, dist, angle) = self;
        let screen_width = 2f64 * dist * (angle / 2f64).tan();
        let screen_heigth = screen_width * (crate::IMAGE_HEIGTH as f64 / crate::IMAGE_WIDTH as f64);
        let left = dir.cross(&up).normalise();
        RayIter {
            screen_top_left: pos
                .add(&dir.scalar_mul(&dist))
                .add(&up.scalar_mul(&(screen_heigth / 2f64)))
                .add(&left.scalar_mul(&(screen_width / 2f64))),
            camera_pos: *pos,
            step_right: left
                .scalar_mul(&(-1 as f64))
                .scalar_mul(&(screen_width / (crate::IMAGE_WIDTH as f64))),
            step_down: up
                .scalar_mul(&(-1 as f64))
                .scalar_mul(&(screen_heigth / (crate::IMAGE_HEIGTH as f64))),
            col: 0,
            row: 0,
        }
    }

    pub fn render(&self, scene: Arc<Scene>, nrays: u32, depth: u8) -> RgbImage {
        println!("Starting render");
        let t0 = Instant::now();
        let mut res = RgbImage::new(crate::IMAGE_WIDTH, crate::IMAGE_HEIGTH);
        let (tx, rx) = mpsc::channel();
        self.render_rays(self.rays(), scene, nrays, depth, tx);
        println!("finished tracing");
        for ((row, col), (r, g, b)) in rx.iter() {
            res.put_pixel(col, row, Rgb([r, g, b]));
        }
        println!("Render complete in {}ms", t0.elapsed().as_millis());
        res
    }

    fn render_rays(
        &self,
        rays: RayIter,
        scene: Arc<Scene>,
        nrays: u32,
        depth: u8,
        tx: mpsc::Sender<((u32, u32), (u8, u8, u8))>,
    ) {
        let tpool = threadpool::Builder::new()
            .num_threads(crate::N_THREADS)
            .thread_stack_size(8000000)
            .build();
        for (ray, row, col) in rays {
            let ntx = tx.clone();
            let nsp = scene.clone();
            tpool.execute(move || {
                let mut tot_colour = zero();
                for _ in 0..nrays {
                    tot_colour = tot_colour.add(&ray.colour(&*nsp, depth));
                }
                let Vect(r, g, b) = tot_colour;
                let r = min(255u8, (r / (nrays as f64)).powf(GAMMA) as u8);
                let g = min(255u8, (g / (nrays as f64)).powf(GAMMA) as u8);
                let b = min(255u8, (b / (nrays as f64)).powf(GAMMA) as u8);
                let res = ntx.send(((row, col), (r, g, b)));
                match res {
                    Ok(()) => (),
                    Err(e) => std::panic!("Thread failed with {}", e),
                };
            });
        }
        tpool.join();
    }
}

pub struct RayIter {
    screen_top_left: Vect,
    camera_pos: Vect,
    step_right: Vect,
    step_down: Vect,
    row: u32,
    col: u32,
}

impl Iterator for RayIter {
    //           Ray  row  col
    type Item = (Ray, u32, u32);

    fn next(&mut self) -> Option<(Ray, u32, u32)> {
        if self.row >= crate::IMAGE_HEIGTH {
            return None;
        };
        let (curr_row, curr_col) = (self.row, self.col);
        let ray = Ray(
            self.camera_pos,
            self.screen_top_left
                .add(&self.step_down.scalar_mul(&(self.row as f64)))
                .add(&self.step_right.scalar_mul(&(self.col as f64)))
                .sub(&self.camera_pos)
                .normalise(),
        );
        self.col += 1;
        if self.col == crate::IMAGE_WIDTH {
            if self.row < crate::IMAGE_HEIGTH {
                self.col = 0;
                self.row += 1;
            }
        }
        Some((ray, curr_row, curr_col))
    }
}
