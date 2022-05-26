//! This module provides basic vector arithmetic for the 3d space we'll be
//! rendering in. Everything is in a functional style so that arguments are
//! borrowed (immutable) and the result is a brand new vector or float.

/// Simple vector type that stores 3 floats, the x, y and z coordinates
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vect(pub f64, pub f64, pub f64);

pub fn zero() -> Vect {
    Vect(0f64, 0f64, 0f64)
}

impl Vect {
    /// Vector addition
    pub fn add(&self, &v2: &Vect) -> Vect {
        let Vect(v11, v12, v13) = self;
        let Vect(v21, v22, v23) = v2;
        Vect(v11 + v21, v12 + v22, v13 + v23)
    }

    /// Vector subtraction
    pub fn sub(&self, v2: &Vect) -> Vect {
        let Vect(v11, v12, v13) = self;
        let Vect(v21, v22, v23) = v2;
        Vect(v11 - v21, v12 - v22, v13 - v23)
    }

    /// Scalar multiplication
    pub fn scalar_mul(&self, s: &f64) -> Vect {
        let Vect(v1, v2, v3) = self;
        Vect(s * v1, s * v2, s * v3)
    }

    /// Scalar division
    pub fn scalar_div(&self, s: &f64) -> Option<Vect> {
        if s < &crate::EPSILON {
            None
        } else {
            let Vect(v1, v2, v3) = self;
            Some(Vect(v1 / s, v2 / s, v3 / s))
        }
    }

    /// Pointwise multiplication
    pub fn pointwise_mul(&self, v2: &Vect) -> Vect {
        let Vect(v11, v12, v13) = self;
        let Vect(v21, v22, v23) = v2;
        Vect(v11 * v21, v12 * v22, v13 * v23)
    }

    /// Dot product
    pub fn dot(&self, v2: &Vect) -> f64 {
        let Vect(v11, v12, v13) = self;
        let Vect(v21, v22, v23) = v2;
        v11 * v21 + v12 * v22 + v13 * v23
    }

    /// Cross product
    pub fn cross(&self, v2: &Vect) -> Vect {
        let Vect(v11, v12, v13) = self;
        let Vect(v21, v22, v23) = v2;
        Vect(
            v12 * v23 - v13 * v22,
            v13 * v21 - v11 * v23,
            v11 * v22 - v12 * v21,
        )
    }

    /// Norm squared of a vector. Faster than the norm, so use if possible
    pub fn norm_sq(&self) -> f64 {
        self.dot(self)
    }

    /// Norm of a vector
    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }

    /// Produce a normalised vector: same direction, norm == 1.
    pub fn normalise(&self) -> Vect {
        match self.scalar_div(&self.norm()) {
            None => *self,
            Some(u) => u,
        }
    }
}

impl std::fmt::Display for Vect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Vect(v1, v2, v3) = self;
        write!(f, "V({}, {}, {})", v1, v2, v3)
    }
}

#[test]
fn vect_test() {
    //Equality test
    assert_eq!(Vect(0.0, 0.0, 0.0) == Vect(0.0, 0.0, 0.0), true);
    assert_eq!(Vect(0.0, 0.0, 0.0) != Vect(10.0, 0.0, 0.0), true);
    //Addition test
    assert_eq!(
        Vect(1.0, 2.0, 3.0).add(&Vect(2.0, 1.0, 0.0)),
        Vect(3.0, 3.0, 3.0)
    );
    assert_eq!(
        Vect(5.0, 5.0, 5.0).add(&Vect(-5.0, -5.0, -5.0)),
        Vect(0.0, 0.0, 0.0)
    );

    assert_eq!(
        Vect(1.0, 2.0, 3.0).sub(&Vect(2.0, 1.0, 0.0)),
        Vect(-1.0, 1.0, 3.0)
    );
    assert_eq!(
        Vect(5.0, 5.0, 5.0).sub(&Vect(-5.0, -5.0, -5.0)),
        Vect(10.0, 10.0, 10.0)
    );

    assert_eq!(Vect(2.0, 1.0, 0.0).scalar_mul(&5f64), Vect(10.0, 5.0, 0.0));
    assert_eq!(
        Vect(-5.0, -5.0, -5.0).scalar_mul(&0f64),
        Vect(0.0, 0.0, 0.0)
    );

    assert_eq!(
        Vect(4.0, 4.0, 4.0).scalar_div(&2f64).unwrap(),
        Vect(2.0, 2.0, 2.0)
    );
    assert_eq!(None, Vect(5.0, 5.0, 5.0).scalar_div(&0f64));

    assert_eq!(Vect(1.0, 1.0, 0.0).dot(&Vect(0.0, 0.0, 1.0)), 0.0);
    assert_eq!(Vect(2.0, 0.0, 0.0).dot(&Vect(2.0, 0.0, 0.0)), 4.0);

    assert_eq!(
        Vect(2.0, 0.0, 0.0).cross(&Vect(2.0, 0.0, 0.0)),
        Vect(0.0, 0.0, 0.0)
    );
    assert_eq!(
        Vect(1.0, 0.0, 0.0).cross(&Vect(0.0, 1.0, 0.0)),
        Vect(0.0, 0.0, 1.0)
    );

    assert_eq!(Vect(1.0, 0f64, 0f64).norm(), 1f64);
    assert_eq!(Vect(3.0, 0.0, 0.0).normalise(), Vect(1.0, 0.0, 0.0));
}
