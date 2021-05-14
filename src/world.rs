use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct World<T: Copy> {
    objects: Vec<Box<dyn Hittable<T>>>,
}

impl<T: Copy> World<T> {
    pub fn new() -> Self {
        World {
            objects: Vec::new(),
        }
    }

    pub fn add<H: Hittable<T> + 'static>(&mut self, hittable: H) {
        self.objects.push(Box::new(hittable));
    }

    pub fn trace(
        &self,
        ray: &Ray<T>,
        t_min: T,
        t_max: T,
    ) -> Option<(HitRecord<T>, &dyn Hittable<T>)> {
        let mut hit: Option<(HitRecord<T>, &dyn Hittable<T>)> = None;

        for i in 0..self.objects.len() {
            let hittable = &self.objects[i];

            let t_max = if let Some((ref rec, _)) = hit {
                rec.t
            } else {
                t_max
            };

            if let Some(rec) = hittable.is_hit(ray, t_min, t_max) {
                hit = Some((rec, hittable.as_ref()));
            }
        }

        hit
    }
}
