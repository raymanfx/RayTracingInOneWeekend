use crate::ray::{Point3, Ray};
use crate::rtweekend;
use crate::vec3::Vec3;

/// Simple virtual camera.
pub struct Camera {
    // width, not adjusted for FOV
    width: f64,
    // height, not adjusted for FOV
    height: f64,
    // vertical field of view (FOV) in radians
    vfov: f64,
    lookfrom: Point3<f64>,
    lookat: Point3<f64>,
    up: Point3<f64>,
    lower_left_corner: Point3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
}

impl Camera {
    /// Create a new pinhole camera.
    ///
    ///   y
    ///   ^
    ///   |   /|
    ///   |  / | h
    ///   | /  |
    ///   |/   |
    ///   ---------------> -z
    ///   \
    ///    \
    ///     \
    ///      \
    ///
    /// θ is the vertical FOV angle in radians:
    ///
    ///     h = tan(θ/2)
    ///
    /// * `width` - Viewport width.
    /// * `height` - Viewport height.
    /// * `focal_length` - Focal length.
    pub fn new(width: f64, height: f64) -> Self {
        // "eye" of the scene aka camera center
        let lookfrom: Vec3<f64> = Vec3::new(0.0, 0.0, 0.0);
        // point that we look at
        let lookat: Vec3<f64> = Vec3::new(0.0, 0.0, -1.0);
        // a vector that points straight into the sky (perpendicular to the ground)
        let up: Vec3<f64> = Vec3::new(0.0, 1.0, 0.0);

        // default vertical FOV: 90 degrees, so that h = 1
        let vfov = rtweekend::degrees_to_radians(90.0);

        let mut camera = Camera {
            width,
            height,
            vfov,
            lookfrom,
            lookat,
            up,
            lower_left_corner: Vec3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(0.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 0.0, 0.0),
        };
        camera.update_perspective();

        camera
    }

    /// Adjusts the camera placement.
    ///
    /// * `lookfrom`: New camera position.
    pub fn lookfrom(mut self, lookfrom: Vec3<f64>) -> Self {
        self.lookfrom = lookfrom;
        self.update_perspective();
        self
    }

    /// Adjusts the camera direction.
    ///
    /// * `lookat`: New camera target point.
    pub fn lookat(mut self, lookat: Vec3<f64>) -> Self {
        self.lookat = lookat;
        self.update_perspective();
        self
    }

    /// Adjusts the camera direction.
    ///
    /// * `up`: New up vector (pointing towards the sky).
    pub fn up(mut self, up: Vec3<f64>) -> Self {
        self.up = up;
        self.update_perspective();
        self
    }

    /// Adjusts the vertical field of view.
    ///
    /// * `up`: New up vector (pointing towards the sky).
    pub fn vfov(mut self, degrees: f64) -> Self {
        let radians = rtweekend::degrees_to_radians(degrees);
        self.vfov = radians;
        self.update_perspective();
        self
    }

    /// Returns the ray for a given horizontal/vertical offset.
    pub fn ray(&self, u: f64, v: f64) -> Ray<f64> {
        Ray::new(
            self.lookfrom,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.lookfrom,
        )
    }

    fn update_perspective(&mut self) {
        // adjust viewport width and height for field of view (FOV)
        let aspect_ratio = self.width / self.height;
        let h = (self.vfov / 2.0).tan();
        let height = self.height * h;
        let width = height * aspect_ratio;

        // get a vector facing in the direction of the negative z axis
        let w = (self.lookfrom - self.lookat).normalized();
        // right hand rule: cross product a x b yields a vector perpendicular to a and b
        let u = Vec3::cross(&self.up, &w).normalized();
        // apply the right hand rule once more
        let v = Vec3::cross(&w, &u);

        // right handed coordinate system: x axis is horizontal
        let horizontal = u * width as f64;
        // right handed coordinate system: y axis is vertical
        let vertical = v * height as f64;
        // do the following to get to the lower left corner:
        //  1. go left as far as possible (half the viewport)
        //  2. go down as far as possible (half the viewport)
        //  3. move forward (negative z direction) so we lay flat on the surface
        let lower_left_corner = self.lookfrom - horizontal / 2.0 - vertical / 2.0 - w;

        self.horizontal = horizontal;
        self.vertical = vertical;
        self.lower_left_corner = lower_left_corner;
    }
}
