use std::ops;

use crate::vec3::Vec3;

/// A point in 3D space.
pub type Point3<T> = Vec3<T>;

/// A ray can be represented as function P(t) = A + t*b:
///
///     * P is a 3D position along a line in 3D space
///     * A is the ray origin
///     * b is the ray direction
///
/// The parameter t moves the point P along the ray. For positive values of t, you move from A into
/// the direction of b and vice versa.
pub struct Ray<T: Copy> {
    origin: Point3<T>,
    direction: Vec3<T>,
}

impl<T: Copy> Ray<T> {
    /// Create a new ray in 3D space.
    ///
    /// * `origin` - Origin of the ray.
    /// * `direction` - Direction in 3D space (x/y/z).
    pub fn new(origin: Point3<T>, direction: Vec3<T>) -> Ray<T> {
        Ray { origin, direction }
    }

    /// Returns the origin.
    pub fn origin(&self) -> Point3<T> {
        self.origin
    }

    /// Returns the direction.
    pub fn direction(&self) -> Vec3<T> {
        self.direction
    }
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>> Ray<T> {
    /// Returns the position on the ray given t.
    ///
    /// The ray equation is: P(t) = A + t*b, where
    ///     * P is a 3D position along a line in 3D space
    ///     * A is the ray origin
    ///     * b is the ray direction
    pub fn at(&self, t: T) -> Point3<T> {
        self.origin + self.direction * t
    }
}
