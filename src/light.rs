use crate::ray::Ray;
use crate::typedefs::{Intersection, Scene};
use crate::vect::*;
use std::f64::consts::PI;

const PI_SQ: f64 = PI * PI;

pub struct Pointlight {
    pub pos: Vect,
    pub intensity: f64,
}

pub trait Light {
    fn get_contribution(&self, intersection: &Intersection, scene: &Scene) -> f64;
}

impl Light for Pointlight {
    fn get_contribution(&self, intersection: &Intersection, scene: &Scene) -> f64 {
        let d_vec = self.pos.sub(&intersection.pos);
        let d_squared = d_vec.norm_sq();
        let d = d_squared.sqrt();
        let d_vec = d_vec.scalar_div(&d).unwrap();
        let shifted_pos = intersection
            .pos
            .add(&intersection.normal.scalar_mul(&crate::EPSILON));
        let ip_to_light = Ray(shifted_pos, d_vec);
        for geo in &scene.0 {
            let block_check = geo.intersect(&ip_to_light);
            if block_check.normal != zero() {
                if block_check.pos.sub(&shifted_pos).norm() < d {
                    return 0f64;
                }
            }
        }
        let angle_contribution = intersection.normal.dot(&d_vec);
        (self.intensity * angle_contribution) / (4f64 * PI_SQ * d_squared)
    }
}
