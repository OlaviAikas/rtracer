use crate::light::Pointlight;
use crate::plane::Plane;
use crate::sphere::Sphere;
use crate::typedefs::{Material, Scene};
use crate::vect::Vect;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

#[derive(Deserialize)]
struct SphereLoader {
    position: [f64; 3],
    radius: f64,
    material: String,
    colour: Option<[f64; 3]>,
}

#[derive(Deserialize)]
struct PlaneLoader {
    point: [f64; 3],
    normal: [f64; 3],
    material: String,
    colour: Option<[f64; 3]>,
}

#[derive(Deserialize)]
struct PointlightLoader {
    position: [f64; 3],
    intensity: f64,
}

#[derive(Deserialize)]
struct SceneLoader {
    sphere: Option<Vec<SphereLoader>>,
    plane: Option<Vec<PlaneLoader>>,
    point_light: Option<Vec<PointlightLoader>>,
}

pub fn load_scene(filename: &str) -> Result<Scene, Error> {
    let mut f = File::open(filename)?;
    let mut buffer = [0u8; 4096];
    let n = f.read(&mut buffer)?;
    let mut s = String::new();
    for i in 0..n {
        s.push(buffer[i] as char);
    }
    let decoded: SceneLoader = toml::from_str(&s).unwrap();
    let mut scene: Scene = (Vec::new(), Vec::new());
    match decoded.sphere {
        None => (),
        Some(spheres) => {
            for sphere_loader in spheres {
                match &sphere_loader.material.as_str() {
                    &"Lambertian" => match sphere_loader.colour {
                        None => {
                            return Err(Error::new(
                                ErrorKind::Other,
                                "Lambertian materials must also specify colour",
                            ))
                        }
                        Some(c) => scene.0.push(Box::new(Sphere {
                            pos: Vect(
                                sphere_loader.position[0],
                                sphere_loader.position[1],
                                sphere_loader.position[2],
                            ),
                            radius: sphere_loader.radius,
                            material: Material::Lambertian(Vect(c[0], c[1], c[2])),
                        })),
                    },
                    _ => return Err(Error::new(ErrorKind::Other, "Invalid material type")),
                }
            }
        }
    }
    match decoded.plane {
        None => (),
        Some(planes) => {
            for plane_loader in planes {
                match &plane_loader.material.as_str() {
                    &"Lambertian" => match plane_loader.colour {
                        None => {
                            return Err(Error::new(
                                ErrorKind::Other,
                                "Lambertian materials must also specify colour",
                            ))
                        }
                        Some(c) => scene.0.push(Box::new(Plane {
                            point: Vect(
                                plane_loader.point[0],
                                plane_loader.point[1],
                                plane_loader.point[2],
                            ),
                            normal: Vect(
                                plane_loader.normal[0],
                                plane_loader.normal[1],
                                plane_loader.normal[2],
                            ),
                            material: Material::Lambertian(Vect(c[0], c[1], c[2])),
                        })),
                    },
                    _ => return Err(Error::new(ErrorKind::Other, "Invalid material type")),
                }
            }
        }
    }
    match decoded.point_light {
        None => (),
        Some(point_lights) => {
            for pointlight_loader in point_lights {
                scene.1.push(Box::new(Pointlight {
                    pos: Vect(
                        pointlight_loader.position[0],
                        pointlight_loader.position[1],
                        pointlight_loader.position[2],
                    ),
                    intensity: pointlight_loader.intensity,
                }));
            }
        }
    }
    Ok(scene)
}
