use crate::geometry::*;
use crate::ray::Ray;
use crate::typedefs::*;
use crate::vect::*;

pub struct Sphere {
    pub pos: Vect,
    pub radius: f64,
    pub material: Material,
    pub albedo: Vect,
}

impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let Ray(rpos, rdir) = ray;
        let p = rdir.dot(&rpos.sub(&self.pos));
        let ocn = rpos.sub(&self.pos).norm();
        let discr = p * p - (ocn * ocn - self.radius * self.radius);
        if discr < 0f64 {
            return Intersection {
                pos: zero(),
                normal: zero(),
            };
        }
        if discr == 0f64 {
            let sol = -p;
            if sol >= 0f64 {
                let int_pos = rpos.add(&rdir.scalar_mul(&sol));
                return Intersection {
                    pos: int_pos,
                    normal: int_pos.sub(&self.pos).normalise(),
                };
            }
            return Intersection {
                pos: zero(),
                normal: zero(),
            };
        }
        let sol1 = -p - discr.sqrt();
        let sol2 = -p + discr.sqrt();
        if sol1 >= 0f64 {
            //Both solutions in front of ray origin
            let int_pos = rpos.add(&rdir.scalar_mul(&sol1));
            return Intersection {
                pos: int_pos,
                normal: int_pos.sub(&self.pos).normalise(),
            };
        }
        if sol2 >= 0f64 {
            //Only second solution in front, ray origin inside the sphere
            let int_pos = rpos.add(&rdir.scalar_mul(&sol2));
            return Intersection {
                pos: int_pos,
                normal: int_pos.sub(&self.pos).normalise(),
            };
        }
        // Both solutions behind the ray origin
        return Intersection {
            pos: zero(),
            normal: zero(),
        };
    }

    fn get_albedo(&self, _point: &Vect) -> Vect {
        self.albedo
    }
    fn get_material(&self) -> Material {
        self.material
    }
}

#[test]
fn sphere_intersection_test() {
    let r1 = Ray(zero(), Vect(1f64, 0f64, 0f64));
    let r2 = Ray(zero(), Vect(0f64, 1f64, 0f64));
    let s = Sphere {
        pos: Vect(10f64, 0f64, 0f64),
        radius: 5f64,
        material: Material::Lambertian,
        albedo: Vect(0.0, 0.0, 0.0),
    };
    assert_ne!(s.intersect(&r1).normal, zero());
    assert_eq!(s.intersect(&r2).normal, zero());
}
