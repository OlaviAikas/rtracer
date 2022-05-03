//! This module provides basic vector arithmetic for the 3d space we'll be
//! rendering in. Everything is in a functional style so that arguments are
//! borrowed (immutable) and the result is a brand new vector or float.

/// Simple vector type that stores 3 floats, the x, y and z coordinates
pub type Vect3 = (f64, f64, f64);

/// Vector addition
pub fn add(v1: &Vect3, &v2: &Vect3) -> Vect3 {
    (v1.0 + v2.0, v1.1 + v2.1, v1.2 + v2.2)
}

/// Vector subtraction
pub fn sub(v1: &Vect3, v2: &Vect3) -> Vect3 {
    (v1.0 - v2.0, v1.1 - v2.1, v1.2 - v2.2)
}

/// Scalar multiplication
pub fn scalar_mul(s: &f64, v: &Vect3) -> Vect3 {
    (s * v.0, s * v.1, s * v.2)
}

/// Scalar division
pub fn scalar_div(s: &f64, v: &Vect3) -> Option<Vect3> {
    if s < &crate::EPSILON {
        None
    } else {
        Some((v.0 / s, v.1 / s, v.2 / s))
    }
}

/// Dot product
pub fn dot(v1: &Vect3, v2: &Vect3) -> f64 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

/// Cross product
pub fn cross(v1: &Vect3, v2: &Vect3) -> Vect3 {
    (
        v1.1 * v2.2 - v1.2 * v2.1,
        v1.2 * v2.0 - v1.0 * v2.2,
        v1.0 * v2.1 - v1.1 * v2.0,
    )
}

/// Norm squared of a vector. Faster than the norm, so use if possible
pub fn norm_sq(v: &Vect3) -> f64 {
    dot(v, v)
}

/// Norm of a vector
pub fn norm(v: &Vect3) -> f64 {
    norm_sq(v).sqrt()
}

/// Produce a normalised vector: same direction, norm == 1.
pub fn normalise(v: &Vect3) -> Vect3 {
    match scalar_div(&norm(v), v) {
        None => *v,
        Some(u) => u,
    }
}

#[test]
fn vect3_test() {
    //Addition test
    assert_eq!(add(&(1.0, 2.0, 3.0), &(2.0, 1.0, 0.0)), (3.0, 3.0, 3.0));
    assert_eq!(add(&(5.0, 5.0, 5.0), &(-5.0, -5.0, -5.0)), (0.0, 0.0, 0.0));

    assert_eq!(sub(&(1.0, 2.0, 3.0), &(2.0, 1.0, 0.0)), (-1.0, 1.0, 3.0));
    assert_eq!(
        sub(&(5.0, 5.0, 5.0), &(-5.0, -5.0, -5.0)),
        (10.0, 10.0, 10.0)
    );

    assert_eq!(scalar_mul(&5.0, &(2.0, 1.0, 0.0)), (10.0, 5.0, 0.0));
    assert_eq!(scalar_mul(&0.0, &(-5.0, -5.0, -5.0)), (0.0, 0.0, 0.0));

    assert_eq!(
        scalar_div(&2f64, &(4.0, 4.0, 4.0)).unwrap(),
        (2.0, 2.0, 2.0)
    );
    assert_eq!(None, scalar_div(&0f64, &(5.0, 5.0, 5.0)));

    assert_eq!(dot(&(1.0, 1.0, 0.0), &(0.0, 0.0, 1.0)), 0.0);
    assert_eq!(dot(&(2.0, 0.0, 0.0), &(2.0, 0.0, 0.0)), 4.0);

    assert_eq!(cross(&(2.0, 0.0, 0.0), &(2.0, 0.0, 0.0)), (0.0, 0.0, 0.0));
    assert_eq!(cross(&(1.0, 0.0, 0.0), &(0.0, 1.0, 0.0)), (0.0, 0.0, 1.0));

    assert_eq!(norm(&(1.0, 0f64, 0f64)), 1f64);
    assert_eq!(normalise(&(3.0, 0.0, 0.0)), (1.0, 0.0, 0.0));
}
