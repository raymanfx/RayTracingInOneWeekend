use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::material::Material;
use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;

pub trait Hittable<T: Copy> {
    /// Test whether the object is hit by an incoming ray.
    ///
    /// * `ray`: Incoming ray of light.
    /// * `t_min`: Minimum depth of the ray.
    /// * `t_max`: Maxmimum depth of the ray.
    fn is_hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>>;

    /// Returns the material of the object.
    fn material(&self) -> &dyn Material<T>;
}

pub struct HitRecord<T: Copy> {
    pub point: Point3<T>,
    pub normal: Vec3<T>,
    pub t: T,
    pub front_face: bool,
}

impl<T: Copy> HitRecord<T>
where
    T: Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Sub<Output = T>
        + Into<f64>,
{
    /// Create a new hit record.
    ///
    /// The normal always points against the incident ray.
    ///
    /// * `point` - Point where the ray hits the object.
    /// * `outward_normal` - Surface normal pointing away from the center of the object.
    /// * `t` - Ray position parameter.
    /// * `ray` - The ray which hits the object.
    pub fn new(point: Point3<T>, outward_normal: Vec3<T>, t: T, ray: &Ray<T>) -> Self {
        // The outward normal always points away from the object. In case the ray is inside the
        // object and hits the hull of the object, the normal would point into the same direction
        // as the ray. We avoid this by later changing the normal stored in this hit record to
        // always point against the ray.
        let dot = Vec3::<T>::dot(&ray.direction(), &outward_normal);
        // Reminder: the dot product is negative if two vectors point into opposite directions,
        // i.e. 90° < theta <= 180°.
        let front_face = dot.into() < 0.0;
        // We set up the surface normal so that it always points against the incident ray.
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }
}
