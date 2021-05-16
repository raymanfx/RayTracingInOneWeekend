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
    /// Fuziness of the specular reflections.
    fuzz: f64,
}

impl Metal {
    /// Create a new metallic material from a given intrinsic object color.
    ///
    /// * `albedo`: Intrinsic surface color.
    /// * `fuzz`: Fuzziness factor for specular reflection in the range [0.0, 1.0].
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: rtweekend::clamp(fuzz, 0.0, 1.0),
        }
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
        // apply fuzzing
        let direction = direction + rtweekend::random_vec_in_unit_sphere() * self.fuzz;
        let scatter = Ray::new(rec.point, direction);

        if Vec3::dot(&scatter.direction(), &rec.normal) <= 0.0 {
            None
        } else {
            Some((scatter, self.albedo))
        }
    }
}

/// Clear (dielectrics) material.
///
/// Examples of dielectric materials are water, glass or diamond.
/// When such a material it hit by a light ray, the ray is split into a reflected and a refracted
/// (transmitted) ray.
///
/// Refraction for dielectrics is described by Snell's law:
///
///     η⋅sinθ = η′⋅sinθ′
///
/// where
///     * θ/θ′: angles from the surface normal
///     * η/η′: refractive indices (e.g. 1.0 for air, 1.3-1.7 for glass)
///
///           R   N
///            \  ^
///             \ |
///       η      v|
/// S ------------x------------
///       η′      |\
///               | \
///               |  \
///               v   v
///               N´   R´
///
/// In the illustration above, R is the incident ray and N is the surface normal. θ is thus the
/// angle between R and N, while θ′ is the angle between N´ and R´. In the illustration, the angles
/// θ and θ′ are exactly the same, so the ray R would pass from air (η = 1.0) through air
/// (η′ = 1.0).
///
/// To calculate the angle θ′, we solve for sinθ′:
///
///     sinθ′ = (η / η′)⋅sinθ
///
/// We split R´ into two parts: one that is perpendicular to N´ and one that is parallel to N´:
///
///     R´ = R′⊥ + R′∥
///
/// Solving for those parts yields:
///
///     R′⊥ = (η / η′)⋅(R + cosθ⋅n)
///     R′∥ = - sqrt(1 - |R′⊥|²)⋅n
///
/// The next step is solving for cosθ. The dot product of two vectors can be expressed in terms of
/// the cosine of the angle between them:
///
///     a⋅b = |a||b| cosθ
///
/// or, assuming unit vectors:
///
///     a⋅b = cosθ
///
/// Thus, we can rewrite R′⊥ as:
///
///     R′⊥ = (η / η′)⋅(R + (-R⋅n)n)
///
/// Sometimes, the refraction ratio η / η′ is too high (e.g. when a ray passes through glass and
/// enters air), so a real solution to Snell's law does not exist. An example:
///
///     sinθ′ = (η / η′)⋅sinθ
///
/// given η = 1.5 (glass) and η´ = 1.0 (air):
///
///     sinθ′ = (1.5 / 1.0)⋅sinθ
///
/// Since sinθ′ can at maximum be 1.0, sinθ must at maximum be (1.0 / 1.5), otherwise the equation
/// can no longer be satisfied. We can solve for sinθ using the following:
///
///     sinθ = sqrt(1 - cos²θ)
///     cosθ = R⋅n
///
/// which yields:
///
///     sinθ = sqrt(1 - (R⋅n)²)
///
/// In case of sinθ > (1.0 / refraction ratio), we cannot refract and thus must reflect. This is
/// called "total internal reflection".
pub struct Dielectric {
    /// Refraction index.
    refraction: f64,
}

impl Dielectric {
    /// Create a new metallic material from a given intrinsic object color.
    ///
    /// * `refraction`: Refraction index.
    pub fn new(refraction: f64) -> Self {
        Dielectric { refraction }
    }

    /// Returns the refracted (trasmitted) ray.
    ///
    /// Based on Snell's law.
    ///
    /// * `ray`: Incident ray.
    /// * `normal`: Surface normal (in eta direction).
    /// * `refraction_ratio`: Refractive ratio (η over η´).
    pub fn refract(ray: &Vec3<f64>, normal: &Vec3<f64>, refraction_ratio: f64) -> Vec3<f64> {
        // the part of the refracted ray which is perpendicular to R´
        let mut cos_theta = Vec3::dot(&(-(*ray)), normal);
        if cos_theta > 1.0 {
            cos_theta = 1.0;
        }
        let perpendicular = (*ray + *normal * cos_theta) * refraction_ratio;
        // the part that is parallel to R´
        let parallel = *normal * (-((1.0 - perpendicular.length_squared()).abs()).sqrt());

        perpendicular + parallel
    }
}

impl Material<f64> for Dielectric {
    fn scatter(&self, ray: &Ray<f64>, rec: &HitRecord<f64>) -> Option<(Ray<f64>, Color)> {
        // assume the material where the ray originates from is air
        let eta = 1.0;
        let eta_prime = self.refraction;
        let refraction_ratio = if rec.front_face {
            eta / eta_prime
        } else {
            eta_prime
        };

        // Total internal reflection: if
        //
        //  (η / η′)⋅sinθ > 1.0
        //
        // we must not refract (and have to reflect) instead!
        let r = ray.direction().normalized();
        // cosθ = R⋅n
        let mut cos_theta = Vec3::dot(&(-(r)), &rec.normal);
        if cos_theta > 1.0 {
            cos_theta = 1.0;
        }
        // sinθ = sqrt(1 - cos²θ)
        let sin_theta = 1.0 - cos_theta * cos_theta;

        // direction of the scattered ray
        let direction = if refraction_ratio * sin_theta > 1.0 {
            // must reflect
            Metal::reflect(&r, &rec.normal)
        } else {
            // can refract
            Dielectric::refract(&r, &rec.normal, refraction_ratio)
        };
        let scatter = Ray::new(rec.point, direction);
        // attenuation is always 1 since air/glass/diamond do not absorb
        let attenuation = Color::new(1.0, 1.0, 1.0);

        Some((scatter, attenuation))
    }
}
