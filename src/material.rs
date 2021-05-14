use std::ops::{Add, Div, Mul, Sub};

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend;
use crate::vec3::Vec3;

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

/// Metal (specular) material.
///
/// For smooth metal surfaces, light is not randomly scattered. Instead, the angle of the incident
/// ray is equal to that of the specular outgoing ray.
///
///           V   N   ^ ^
///            \  ^  /  |
///             \ | /   | B
///              v|/    |
/// S ------------x------------
///                \    |
///                 \   | B
///                  v  |
///
/// The elements of the figure are:
///     * S: Metal surface
///     * V: Incident ray
///     * N: Surface normal
///
/// Additionally, B is an additional vector for illustration purposes.
/// The reflected ray (in between N and B) has the same angle as the incident ray V with regards to
// the surface. It can be computed as V + B + B.
///
/// In our current design, N is a unit vector, but the same does not have to be true for V.
/// Furthermore, V points inwards, so the sign has to be changed.
/// All this is encoded in the reflect() function.
pub struct Metal {
    /// Color of the object.
    albedo: Color,
}

impl Metal {
    /// Create a new metallic material from a given intrinsic object color.
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }

    /// Returns the reflected ray.
    ///
    /// This basically just encodes the V + B + B term for the specular reflection. We flip the
    /// sign and simplify the term so it becomes V - 2B.
    pub fn reflect<T: Copy>(ray: &Vec3<T>, normal: &Vec3<T>) -> Vec3<T>
    where
        T: Add<Output = T>
            + Div<Output = T>
            + Mul<Output = T>
            + Mul<f64, Output = T>
            + Sub<Output = T>
            + From<f64>
            + Into<f64>,
    {
        let b = *normal * Vec3::dot(ray, normal);
        *ray - b * T::from(2.0)
    }
}

impl Material<f64> for Metal {
    fn scatter(&self, ray: &Ray<f64>, rec: &HitRecord<f64>) -> Option<(Ray<f64>, Color)> {
        // specular reflection
        let direction = Metal::reflect(&ray.direction().normalized(), &rec.normal);
        let scatter = Ray::new(rec.point, direction);

        if Vec3::dot(&scatter.direction(), &rec.normal) <= 0.0 {
            None
        } else {
            Some((scatter, self.albedo))
        }
    }
}
