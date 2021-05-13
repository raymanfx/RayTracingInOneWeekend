use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;

pub struct HitRecord<T: Copy> {
    pub point: Point3<T>,
    pub normal: Vec3<T>,
    pub t: T,
}

pub trait Hittable<T: Copy> {
    fn is_hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>>;
}
