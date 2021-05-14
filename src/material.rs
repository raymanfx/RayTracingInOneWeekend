use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend;

/// Generic material trait.
pub trait Material<T: Copy> {
    /// Scatter an incoming light ray on a material.
    ///
    /// Returns a scattered ray and the attenuation color if there is reflection.
    /// Otherwise, none is returned.
    ///
    /// * `ray` - Incoming light ray.
    /// * `rec` - Previous hit record of the ray on some object.
    fn scatter(&self, ray: &Ray<T>, rec: &HitRecord<T>) -> Option<(Ray<T>, Color)>;
}

/// Lambertian (diffuse) material.
///
/// In our diffuse reflection model, a lambertian material will always both scatter and attenuate
/// by its own reflectance (albedo).
///
/// Should only be used for smooth matte surfaces, not rough matte ones.
/// See https://www.cs.cmu.edu/afs/cs/academic/class/15462-f09/www/lec/lec8.pdf for explanation.
pub struct Lambertian {
    /// Color of the object.
    albedo: Color,
}

impl Lambertian {
    /// Create a new diffuse material from a given intrinsic object color.
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material<f64> for Lambertian {
    fn scatter(&self, _ray: &Ray<f64>, rec: &HitRecord<f64>) -> Option<(Ray<f64>, Color)> {
        // Diffuse reflection: True Lambertian reflection.
        // We aim for a Lambertian distribution of the reflected rays, which has a distribution of
        // cos(phi) instead of cos³(phi) for random vectors inside the unit sphere.
        // To achieve this, we pick a random point on the surface of the unit sphere, which is done
        // by picking a random point inside the sphere and then normalizing that point.
        let random_unit_vec = rtweekend::random_vec_in_unit_sphere().normalized();

        // Diffuse reflection: send out a new ray from the hit position point pointing towards a
        // random point on the surface of the sphere tangent to that hit point.
        // Possible problem: the recursion depth may be too deep, so we blow up the stack. Avoid
        // this by limiting the number of child rays.
        let scatter_direction = rec.normal + random_unit_vec;
        let scatter = Ray::new(rec.point, scatter_direction);

        Some((scatter, self.albedo))
    }
}
