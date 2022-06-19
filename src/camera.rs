use crate::ray::*;
use crate::typedefs::Scene;
use crate::vect::*;
use image::{Rgb, RgbImage};
use std::cmp::{max, min};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Instant;

const GAMMA: f64 = 0.45;

/// The Camera type is a product type that contains the position of the
/// camera, the direction its facing, the direction that's "up" from the
/// camera's point of view and the
/// viewing angle of the camera, which determines how much of the world the
/// camera sees.
///           pos, screen_top_left, step_right, step_down
pub struct Camera(Vect, Vect, Vect, Vect);

/// Create a new camera with the given position, direction,
/// up direction and angle. Enforces that dot(dir, up) == 0,
/// and norms pos dir and up just to be safe.
pub fn new(pos: Vect, dir: Vect, up: Vect, angle: f64) -> Camera {
    if dir.dot(&up) != 0f64 {
        panic!(
            "Tried to create camera with non-perpendicular
        facing and up directions"
        );
    }
    let dir = dir.normalise();
    let up = up.normalise();
    let screen_width = 2f64 * (angle / 2f64).tan();
    let screen_heigth = screen_width * (crate::IMAGE_HEIGTH as f64 / crate::IMAGE_WIDTH as f64);
    let left = dir.cross(&up).normalise();
    Camera(
        pos,
        pos.add(&dir)
            .add(&up.scalar_mul(&(screen_heigth / 2f64)))
            .add(&left.scalar_mul(&(screen_width / 2f64))),
        left.scalar_mul(&(-1 as f64))
            .scalar_mul(&(screen_width / (crate::IMAGE_WIDTH as f64))),
        up.scalar_mul(&(-1 as f64))
            .scalar_mul(&(screen_heigth / (crate::IMAGE_HEIGTH as f64))),
    )
}

impl Camera {
    pub fn ray(&self, row: &u32, col: &u32) -> Ray {
        let Camera(pos, screen_top_left, step_right, step_down) = self;
        Ray(
            *pos,
            screen_top_left
                .add(&step_down.scalar_mul(&(*row as f64)))
                .add(&step_right.scalar_mul(&(*col as f64)))
                .sub(&pos)
                .normalise(),
        )
    }

    pub fn render(&self, scene: Arc<Scene>, nrays: u32, depth: u8) -> RgbImage {
        println!("Starting render");
        let t0 = Instant::now();
        let (tx, rx) = mpsc::channel();
        self.render_rays(scene, nrays, depth, tx);
        println!("finished tracing");
        println!("tracing complete in {}ms", t0.elapsed().as_millis());
        let mut res = RgbImage::new(crate::IMAGE_WIDTH, crate::IMAGE_HEIGTH);
        for ((row, col), (r, g, b)) in rx.iter() {
            res.put_pixel(col, row, Rgb([r, g, b]));
        }
        res
    }

    fn render_rays(
        &self,
        scene: Arc<Scene>,
        nrays: u32,
        depth: u8,
        tx: mpsc::Sender<((u32, u32), (u8, u8, u8))>,
    ) {
        let tpool = threadpool::Builder::new()
            .num_threads(crate::N_THREADS)
            .thread_stack_size(8000000)
            .build();
        for row in 0..crate::IMAGE_HEIGTH {
            for col in 0..crate::IMAGE_WIDTH {
                let ray = self.ray(&row, &col);
                let ntx = tx.clone();
                let nsp = scene.clone();
                tpool.execute(move || {
                    let mut tot_colour = zero();
                    for _ in 0..nrays {
                        tot_colour = tot_colour.add(&ray.colour(&*nsp, depth));
                    }
                    let Vect(r, g, b) = tot_colour;
                    let r = max(0u8, min(255u8, (r / (nrays as f64)).powf(GAMMA) as u8));
                    let g = max(0u8, min(255u8, (g / (nrays as f64)).powf(GAMMA) as u8));
                    let b = max(0u8, min(255u8, (b / (nrays as f64)).powf(GAMMA) as u8));
                    let res = ntx.send(((row, col), (r, g, b)));
                    match res {
                        Ok(()) => (),
                        Err(e) => std::panic!("Thread failed with {}", e),
                    };
                });
            }
        }
        tpool.join();
    }
}
