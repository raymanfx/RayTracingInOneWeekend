use std::cmp::PartialOrd;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;

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

pub trait Hittable<T: Copy> {
    fn is_hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>>;
}

pub struct HittableList<T: Copy> {
    objects: Vec<Box<dyn Hittable<T>>>,
}

impl<T: Copy> HittableList<T> {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add<H: Hittable<T> + 'static>(&mut self, hittable: H) {
        self.objects.push(Box::new(hittable));
    }
}

impl<T: Copy> Hittable<T> for HittableList<T>
where
    T: PartialOrd,
{
    fn is_hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>> {
        let mut hit: Option<HitRecord<T>> = None;

        for hittable in &self.objects {
            let t_max = if let Some(ref rec) = hit {
                rec.t
            } else {
                t_max
            };

            if let Some(rec) = hittable.is_hit(ray, t_min, t_max) {
                hit = Some(rec);
            }
        }

        hit
    }
}
