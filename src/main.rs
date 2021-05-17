use std::io;
use std::io::Write;

mod ppm;
use ppm::Image;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::{Point3, Ray};

mod hittable;

mod sphere;
use sphere::Sphere;

mod rtweekend;

mod camera;
use camera::Camera;

mod color;
use color::Color;

mod world;
use world::World;

mod material;

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

/// Compute the color of pixel hit by a ray.
fn ray_color(ray: &Ray<f64>, world: &World<f64>, depth: usize) -> Color {
    if depth == 0 {
        // ray bounce limit exceeded, no more light is reflected
        return Color::new(0.0, 0.0, 0.0);
    }

    // Fix shadow acne: due to floating point approximation, some of the reflected rays hit the
    // object they are reflecting off of not at exactly t = 0, but e.g. t = -0.000001 or
    // t = 0.000001. Ignore hits near zero to work around this.
    let t_min = 0.001;
    let t_max = std::f64::MAX;

    if let Some((rec, material)) = world.trace(ray, t_min, t_max) {
        // scatter the light ray
        if let Some((scatter, attenuation)) = material.scatter(ray, &rec) {
            let mut scatter_color = ray_color(&scatter, world, depth - 1);
            // consider attenuation of the object
            scatter_color[0] = scatter_color[0] * attenuation[0];
            scatter_color[1] = scatter_color[1] * attenuation[1];
            scatter_color[2] = scatter_color[2] * attenuation[2];
            return scatter_color;
        } else {
            // no light is reflected
            return Color::new(0.0, 0.0, 0.0);
        }
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
    eprintln!(
        ">> Viewport: {} (W) x {} (H)",
        VIEWPORT_WIDTH, VIEWPORT_HEIGHT
    );
    let camera = Camera::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT)
        .lookfrom(Vec3::new(-2.0, 2.0, 1.0))
        .lookat(Vec3::new(0.0, 0.0, -1.0))
        .up(Vec3::new(0.0, 1.0, 0.0))
        .vfov(20.0);

    // World
    let mut world = World::new();
    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    let sphere_ground_mat = material::Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let sphere_center_mat = material::Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5);
    let sphere_left_mat = material::Dielectric::new(1.5);
    let sphere_left_inner = Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45);
    let sphere_left_inner_mat = material::Dielectric::new(1.5);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5);
    let sphere_right_mat = material::Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    // add objects to the world
    world.add(sphere_ground, sphere_ground_mat);
    world.add(sphere_center, sphere_center_mat);
    world.add(sphere_left, sphere_left_mat);
    world.add(sphere_left_inner, sphere_left_inner_mat);
    world.add(sphere_right, sphere_right_mat);

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
