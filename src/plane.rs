use crate::geometry::Geometry;
use crate::ray::*;
use crate::typedefs::{Intersection, Material};
use crate::vect::*;

pub struct Plane {
    pub point: Vect,
    pub normal: Vect,
    pub albedo: Vect,
    pub material: Material,
}

impl Geometry for Plane {
    fn intersect(&self, ray: &Ray) -> Intersection {
        // p in ray line = ray.pos + t*ray.dir
        // p in plane = (p - plane.point).dot(plane.normal) == 0
        // substitute:
        // (ray.pos + t*ray.dir - plane.point).dot(plane.normal) == 0
        // dot product bilinear
        // (ray.pos - plane.point).dot(plane.normal) + t(ray.dir.dot(plane.normal)) == 0
        // t = -((ray.pos - plane.point).dot(plane.normal))/ray.dir.dot(plane.normal)
        // If div by 0 then either no intersection or the ray is within the plane
        let Ray(rpos, rdir) = ray;
        let raydirdotplanenormal = rdir.dot(&self.normal);
        if raydirdotplanenormal == 0f64 {
            return Intersection {
                pos: zero(),
                normal: zero(),
            };
        }
        let t = -(rpos.sub(&self.point).dot(&self.normal)) / raydirdotplanenormal;
        if t < 0f64 {
            return Intersection {
                pos: zero(),
                normal: zero(),
            };
        }
        if raydirdotplanenormal < 0f64 {
            Intersection {
                pos: rpos.add(&rdir.scalar_mul(&t)),
                normal: self.normal,
            }
        } else {
            Intersection {
                pos: rpos.add(&rdir.scalar_mul(&t)),
                normal: self.normal.scalar_mul(&-1f64),
            }
        }
    }
    fn get_albedo(&self, _point: &Vect) -> Vect {
        self.albedo
    }
    fn get_material(&self) -> Material {
        self.material
    }
}
