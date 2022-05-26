use crate::ray::Ray;
use crate::typedefs::*;
use crate::vect::*;

pub trait Geometry {
    fn intersect(&self, ray: &Ray) -> Intersection;
    fn get_material(&self) -> Material;
    fn get_albedo(&self, point: &Vect) -> Vect;
}
