/// The Camera type is a product type that contains the position of the
/// camera, the direction its facing, the direction that's "up" from the 
/// camera's point of view, and the distance of the screen from the
/// camera position, e.g the focal distance of the camera.
pub type Camera = (Vect3, Vect3, Vect3, f64);


pub fn generate_rays()