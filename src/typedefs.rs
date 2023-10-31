use crate::geometry::Geometry;
use crate::light::Light;
use crate::vect::*;

pub struct Intersection {
    pub pos: Vect,
    pub normal: Vect,
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Vect), //Albedo
    Mirror,
}

pub type Scene = (
    Vec<Box<dyn Geometry + Send + Sync>>,
    Vec<Box<dyn Light + Send + Sync>>,
);
