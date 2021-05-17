use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;

pub struct World<T: Copy> {
    objects: Vec<(
        Box<dyn Hittable<T> + Send + Sync>,
        Box<dyn Material<T> + Send + Sync>,
    )>,
}

impl<T: Copy> World<T> {
    pub fn new() -> Self {
        World {
            objects: Vec::new(),
        }
    }

    pub fn add<H, M>(&mut self, hittable: H, material: M)
    where
        H: Hittable<T> + Send + Sync + 'static,
        M: Material<T> + Send + Sync + 'static,
    {
        self.objects.push((Box::new(hittable), Box::new(material)));
    }

    pub fn trace(
        &self,
        ray: &Ray<T>,
        t_min: T,
        t_max: T,
    ) -> Option<(HitRecord<T>, &dyn Material<T>)> {
        let mut hit: Option<(HitRecord<T>, &dyn Material<T>)> = None;

        for i in 0..self.objects.len() {
            let (hittable, material) = &self.objects[i];

            let t_max = if let Some((ref rec, _)) = hit {
                rec.t
            } else {
                t_max
            };

            if let Some(rec) = hittable.is_hit(ray, t_min, t_max) {
                hit = Some((rec, material.as_ref()));
            }
        }

        hit
    }
}
