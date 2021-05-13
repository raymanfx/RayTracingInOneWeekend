use crate::hittable::{HitRecord, Hittable};
use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;

pub struct Sphere<T: Copy> {
    center: Point3<T>,
    radius: T,
}

impl<T: Copy> Sphere<T> {
    pub fn new(center: Point3<T>, radius: T) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable<f64> for Sphere<f64> {
    fn is_hit(&self, ray: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord<f64>> {
        // Equation of a sphere with radius r, centered at the origin:
        //      x² + y² + z² = r²
        //
        // If any given point P = (x,y,z) is inside the sphere, then:
        //      x² + y² + z² < r²
        //
        // and accordingly when it is outside the sphere:
        //      x² + y² + z² > r²
        //
        // For a sphere center at an arbitrary point (C_x,C_y,C_z):
        //      (x - C_x)² + (y - C_y)² + (z - C_z)² = r²
        //
        // Since the vector from center C to point P is (P - C), we can write:
        //      (P - C) * (P - C) = (x - C_x)² + (y - C_y)² + (z - C_z)²
        //
        // or in short:
        //      (P - C) * (P - C) = r²
        //
        // which can be read as: "any point P that satisfies this equation is on the sphere".
        // Plugging in the equation for a ray: P(t) = A + t*b, we get the following:
        //      (A + t*b - C) * (A + t*b - C) = r²
        //
        // Expanding this equation and moving all terms to the left side:
        //      t²b * b + 2tb * (A - C) + (A - C) * (A - C) - r² = 0
        //
        // In graphics, the algebra usually related to the geometry. In our case, solving the quadratic
        // equation for t yields a square root part which is either:
        //      positive => two real solutions, two hit points
        //      negative => no real solution, no hit points
        //      zero     => one real solution, one hit point

        // we simplify the code by applying two things:
        //      1. a vector dotted with itself is equal to its squared length
        //      2. b = 2h to remove the factor of two

        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        // The quadratic polynomial ax² + bx + c has discriminant: b² - 4ac.
        // (Wikipedia: https://en.wikipedia.org/wiki/Discriminant)
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let d_sqrt = discriminant.sqrt();

        // find the nearest root that lies in the acceptable range
        let mut root = (-half_b - d_sqrt) / a;
        if root < t_min || root > t_max {
            root = (-half_b + d_sqrt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        // outward surface normal is in the direction of the hit point minus the center
        let outward_normal = (point - self.center) / self.radius;
        Some(HitRecord::new(point, outward_normal, root, ray))
    }
}
