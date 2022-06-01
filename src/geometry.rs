use crate::ray::Ray;
use crate::typedefs::*;

pub trait Geometry {
    fn intersect(&self, ray: &Ray) -> Intersection;
    fn get_material(&self) -> Material;
}
