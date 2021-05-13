use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;

/// Simple virtual camera.
pub struct Camera {
    origin: Point3<f64>,
    lower_left_corner: Point3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
}

impl Camera {
    /// Create a new virtual camera.
    ///
    /// * `width` - Viewport width.
    /// * `height` - Viewport height.
    /// * `focal_length` - Focal length.
    pub fn new(width: f64, height: f64, focal_length: f64) -> Self {
        // "eye" of the scene aka camera center
        let origin = Point3::<f64>::new(0.0, 0.0, 0.0);
        // right handed coordinate system: x axis is horizontal
        let horizontal = Vec3::<f64>::new(width, 0.0, 0.0);
        // right handed coordinate system: y axis is vertical
        let vertical = Vec3::<f64>::new(0.0, height, 0.0);
        // do the following to get to the lower left corner:
        //  1. go left as far as possible (half the viewport)
        //  2. go down as far as possible (half the viewport)
        //  3. move forward (negative z direction) so we lay flat on the surface
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::<f64>::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    /// Returns the ray for a given horizontal/vertical offset.
    pub fn ray(&self, u: f64, v: f64) -> Ray<f64> {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
