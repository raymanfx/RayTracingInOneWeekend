use std::io;
use std::io::Write;

mod ppm;
use ppm::Image;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::{Point3, Ray};

mod hittable;
use hittable::{Hittable, HittableList};

mod sphere;
use sphere::Sphere;

mod rtweekend;

mod camera;
use camera::Camera;

/// RGB color with each channel ranging from 0.0 to 1.0
type Color = Vec3<f64>;

/// Post processing to transform RGB channels into PPM RGB color values.
///
/// We perform two steps:
///     1. Gamma correction using gamma=2
///     2. Color value mapping from [0.0, 1.0] to [0, 255]
fn write_color(color: &Color) {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    // gamma correction: raise color to the power of 1/gamma
    // here: use gamma=2 as first approximation
    r = r.sqrt();
    g = g.sqrt();
    b = b.sqrt();

    // clamp to [0.0, 1.0] range
    r = rtweekend::clamp(r, 0.0, 0.999);
    g = rtweekend::clamp(g, 0.0, 0.999);
    b = rtweekend::clamp(b, 0.0, 0.999);

    // map to [0, 255] range
    r = 256.0 * r;
    g = 256.0 * g;
    b = 256.0 * b;

    println!("{} {} {}", r as u8, g as u8, b as u8);
}

/// Find a random vector in the unit sphere.
fn random_vec_in_unit_sphere() -> Vec3<f64> {
    loop {
        // choose a random vector inside the unit cube
        let x = rtweekend::random(-1.0..1.0);
        let y = rtweekend::random(-1.0..1.0);
        let z = rtweekend::random(-1.0..1.0);
        let vec = Vec3::new(x, y, z);

        if vec.length_squared() >= 1.0 {
            // vector is not inside the unit sphere, continue the search
            continue;
        }

        return vec;
    }
}

/// Compute the color of pixel hit by a ray.
fn ray_color(ray: &Ray<f64>, world: &HittableList<f64>, depth: usize) -> Color {
    if depth == 0 {
        // ray bounce limit exceeded, no more light is reflected
        return Color::new(0.0, 0.0, 0.0);
    }

    // Fix shadow acne: due to floating point approximation, some of the reflected rays hit the
    // object they are reflecting off of not at exactly t = 0, but e.g. t = -0.000001 or
    // t = 0.000001. Ignore hits near zero to work around this.
    let t_min = 0.001;
    let t_max = std::f64::MAX;

    if let Some(rec) = world.is_hit(ray, t_min, t_max) {
        // Diffuse reflection: send out a new ray from the hit position point pointing towards a
        // random direction inside the unit sphere tangent to that hit point.
        // Possible problem: the recursion depth may be too deep, so we blow up the stack. Avoid
        // this by limiting the number of child rays.
        let target = rec.point + rec.normal + random_vec_in_unit_sphere();
        let ray = Ray::new(rec.point, target - rec.point);

        // assume the normal is a unit length vector in the range [-1.0, 1.0] and map it to the
        // [0.0, 1.0] range since we are going to interpret it as RGB
        return ray_color(&ray, world, depth - 1) * 0.5;
    }

    // scale the ray direction to unit length (so -1.0 < y < 1.0)
    let unit_direction = ray.direction().normalized();
    // scale t so 0.0 <= t <= 1.0
    let t = 0.5 * (unit_direction.y() + 1.0);
    // linear blend aka interpolation between white and blue
    let white = Color::new(1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0);
    white * (1.0 - t) + blue * t
}

fn main() -> io::Result<()> {
    // Image settings
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const RAY_SAMPLES_PER_PIXEL: usize = 100;
    const RAY_MAX_DEPTH: usize = 50;
    eprintln!(">> Image: {} (W) x {} (H)", IMAGE_WIDTH, IMAGE_HEIGHT);

    // Camera settings
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;
    eprintln!(
        ">> Viewport: {} (W) x {} (H) - focal: {}",
        VIEWPORT_WIDTH, VIEWPORT_HEIGHT, FOCAL_LENGTH
    );
    let camera = Camera::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT, FOCAL_LENGTH);

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // create the image buffer
    let mut img = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, Color::new(0.0, 0.0, 0.0));

    // fill image with test data
    for j in (0..img.height()).rev() {
        eprint!("\r>> Scanlines remaining: {:width$}", j, width = 5);
        io::stdout().flush()?;

        for i in 0..img.width() {
            let mut color = Color::new(0.0, 0.0, 0.0);

            // For each pixel, we send RAY_SAMPLES_PER_PIXEL number of rays and essentially average
            // their color values to get a final pixel color.
            for _ in 0..RAY_SAMPLES_PER_PIXEL {
                let u = (i as f64 + rtweekend::random(0.0..1.0)) / ((img.width() - 1) as f64);
                let v = (j as f64 + rtweekend::random(0.0..1.0)) / ((img.height() - 1) as f64);
                let ray = camera.ray(u, v);
                color = color + ray_color(&ray, &world, RAY_MAX_DEPTH);
            }

            // divide the color by the number of samples
            let scale = 1.0 / RAY_SAMPLES_PER_PIXEL as f64;
            color = color * scale;

            img[j][i] = color;
        }
    }
    eprintln!("\n>> Render done");

    // print PPM header
    println!("P3");
    println!("{} {}", img.width(), img.height());
    println!("255");
    // print PPM data
    for j in (0..img.height()).rev() {
        for i in 0..img.width() {
            let pix = img[j][i];
            write_color(&pix);
        }
    }

    Ok(())
}
