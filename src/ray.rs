use crate::geometry::Geometry;
use crate::typedefs::{Intersection, Material, Scene};
use crate::vect::*;
use rand::prelude::*;
use std::f64::consts::PI;

const PI2: f64 = PI * PI;

/// A ray (pos, dir) is a half-line in the scene space that starts
/// from pos and goes towards dir.
pub struct Ray(pub Vect, pub Vect);

impl std::fmt::Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Ray(start, dir) = self;
        write!(f, "Ray({}, {})", start, dir)
    }
}

impl Ray {
    pub fn colour(&self, scene: &Scene, depth: u8) -> Vect {
        if depth <= 0u8 {
            return Vect(0.0, 0.0, 0.0);
        }
        let Ray(rpos, _) = &self;
        let mut closest_intersection = Intersection {
            normal: zero(),
            pos: zero(),
        };
        let mut closest_dsquared = std::f64::INFINITY;
        let mut closest_geo: &std::boxed::Box<dyn Geometry + Send + Sync> = &scene.0[0];
        for geo in &scene.0 {
            let intersection = geo.intersect(&self);
            if intersection.normal != zero() {
                let dsquared = rpos.sub(&intersection.pos).norm_sq();
                if dsquared < closest_dsquared {
                    closest_dsquared = dsquared;
                    closest_intersection = intersection;
                    closest_geo = geo;
                }
            }
        }
        if closest_dsquared < std::f64::INFINITY {
            let mut tot_light = 0.0;
            for light in &scene.1 {
                tot_light += light.get_contribution(&closest_intersection, &scene);
            }
            if tot_light <= 0.1 {
                return Vect(255.0, 0.0, 250.0);
            }
            let geo_alb = closest_geo.get_albedo(&closest_intersection.pos);
            let l0 = geo_alb.scalar_mul(&tot_light);
            match closest_geo.get_material() {
                Material::Lambertian => {
                    let rand_dir = box_muller_random_vector(&closest_intersection.normal);
                    let w1 = Ray(closest_intersection.pos, rand_dir);
                    return l0.add(&geo_alb.pointwise_mul(&w1.colour(scene, depth - 1)));
                }
            }
        }
        Vect(50.0, 0.0, 0.0)
    }
}

fn box_muller_random_vector(normal: &Vect) -> Vect {
    let mut rng = rand::thread_rng();
    let r1: f64 = rng.gen();
    let r2: f64 = rng.gen();
    let sqrt1mr2 = (1f64 - r2).sqrt();
    let twopir1 = PI2 * r1;
    let x = (twopir1.cos()) * sqrt1mr2;
    let y = (twopir1.sin()) * sqrt1mr2;
    let z = r2.sqrt();
    let xabs = x.abs();
    let yabs = y.abs();
    let zabs = z.abs();
    let t1 = if xabs <= yabs && xabs <= zabs {
        Vect(0.0, -z, y).normalise()
    } else if yabs <= xabs && yabs <= zabs {
        Vect(-z, 0.0, x).normalise()
    } else {
        Vect(-y, x, 0.0).normalise()
    };
    let t2 = t1.cross(normal);
    t1.scalar_mul(&x)
        .add(&t2.scalar_mul(&y))
        .add(&normal.scalar_mul(&z))
}
