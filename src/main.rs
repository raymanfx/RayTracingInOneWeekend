use std::io;
use std::io::Write;

use rayon::prelude::*;

use minifb::{Window, WindowOptions};

mod ppm;
use ppm::Image;

mod vec;
use vec::Vec3;

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
fn color_to_rgb8(color: &Color) -> [u8; 3] {
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

    [r as u8, g as u8, b as u8]
}

/// Compute the color of pixel hit by a ray.
fn ray_color(ray: &Ray<f64>, world: &World<f64>, depth: usize) -> Color {
    if depth == 0 {
        // ray bounce limit exceeded, no more light is reflected
        return Color::new3(0.0, 0.0, 0.0);
    }

    // Fix shadow acne: due to floating point approximation, some of the reflected rays hit the
    // object they are reflecting off of not at exactly t = 0, but e.g. t = -0.000001 or
    // t = 0.000001. Ignore hits near zero to work around this.
    let t_min = 0.001;
    let t_max = std::f64::MAX;

    if let Some((rec, material)) = world.trace(ray, t_min, t_max) {
        // DEBUG: surface normal shading
        //return Color::new3(
        //    rec.normal.x() + 1.0,
        //    rec.normal.y() + 1.0,
        //    rec.normal.z() + 1.0,
        //) * 0.5;

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
            return Color::new3(0.0, 0.0, 0.0);
        }
    }

    // scale the ray direction to unit length (so -1.0 < y < 1.0)
    let unit_direction = ray.direction().normalized();
    // scale t so 0.0 <= t <= 1.0
    let t = 0.5 * (unit_direction.y() + 1.0);
    // linear blend aka interpolation between white and blue
    let white = Color::new3(1.0, 1.0, 1.0);
    let blue = Color::new3(0.5, 0.7, 1.0);
    white * (1.0 - t) + blue * t
}

/// Setup a random scene.
fn random_scene() -> World<f64> {
    let mut world = World::new();

    let sphere_ground = Sphere::new(Point3::new3(0.0, -1000.0, 0.0), 1000.0);
    let sphere_ground_mat = material::Lambertian::new(Color::new3(0.5, 0.5, 0.5));
    world.add(sphere_ground, sphere_ground_mat);

    for a in -11..11 {
        for b in -11..11 {
            let random = rtweekend::random(0.0..1.0);
            let center = Point3::new3(a as f64 + 0.9 * random, 0.2, b as f64 + 0.9 * random);

            if (center - Point3::new3(4.0, 0.2, 0.0)).length() > 0.9 {
                if random < 0.8 {
                    // diffuse
                    let albedo = Color::new3(
                        rtweekend::random(0.0..1.0) * rtweekend::random(0.0..1.0),
                        rtweekend::random(0.0..1.0) * rtweekend::random(0.0..1.0),
                        rtweekend::random(0.0..1.0) * rtweekend::random(0.0..1.0),
                    );
                    let material = material::Lambertian::new(albedo);
                    let sphere = Sphere::new(center, 0.2);
                    world.add(sphere, material);
                } else if random < 0.95 {
                    // metal
                    let albedo = Color::new3(
                        rtweekend::random(0.5..1.0),
                        rtweekend::random(0.5..1.0),
                        rtweekend::random(0.5..1.0),
                    );
                    let fuzz = rtweekend::random(0.0..0.5);
                    let material = material::Metal::new(albedo, fuzz);
                    let sphere = Sphere::new(center, 0.2);
                    world.add(sphere, material);
                } else {
                    // glass
                    let material = material::Dielectric::new(1.5);
                    let sphere = Sphere::new(center, 0.2);
                    world.add(sphere, material);
                }
            }
        }
    }

    let material = material::Dielectric::new(1.5);
    let sphere = Sphere::new(Point3::new3(0.0, 1.0, 0.0), 1.0);
    world.add(sphere, material);

    let material = material::Lambertian::new(Color::new3(0.4, 0.2, 0.1));
    let sphere = Sphere::new(Point3::new3(-4.0, 1.0, 0.0), 1.0);
    world.add(sphere, material);

    let material = material::Metal::new(Color::new3(0.7, 0.6, 0.5), 0.0);
    let sphere = Sphere::new(Point3::new3(4.0, 1.0, 0.0), 1.0);
    world.add(sphere, material);

    world
}

fn main() -> io::Result<()> {
    // Image settings
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 1200;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const RAY_SAMPLES_PER_PIXEL: usize = 500;
    const RAY_MAX_DEPTH: usize = 50;
    eprintln!(">> Image: {} (W) x {} (H)", IMAGE_WIDTH, IMAGE_HEIGHT);

    // minifb setup
    #[cfg(feature = "minifb")]
    let mut window =
        Window::new("Scene", IMAGE_WIDTH, IMAGE_HEIGHT, WindowOptions::default()).unwrap();
    #[cfg(feature = "minifb")]
    let mut buffer = vec![0u32; IMAGE_WIDTH * IMAGE_HEIGHT];

    // Camera settings
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    eprintln!(
        ">> Viewport: {} (W) x {} (H)",
        VIEWPORT_WIDTH, VIEWPORT_HEIGHT
    );
    let camera = Camera::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT)
        .lookfrom(Vec3::new3(13.0, 2.0, 3.0))
        .lookat(Vec3::new3(0.0, 0.0, 0.0))
        .up(Vec3::new3(0.0, 1.0, 0.0))
        .vfov(20.0)
        .lens(0.1, 10.0);

    // World
    let world = random_scene();

    // create the image buffer
    let mut img = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, Color::new3(0.0, 0.0, 0.0));

    // fill image with test data
    for j in (0..img.height()).rev() {
        eprint!("\r>> Scanlines remaining: {:width$}", j, width = 5);
        io::stdout().flush()?;

        let scanline: Vec<Color> = (0..img.width())
            .into_par_iter()
            .map(|i| {
                let mut color = Color::new3(0.0, 0.0, 0.0);

                // For each pixel, we send RAY_SAMPLES_PER_PIXEL number of rays and essentially average
                // their color values to get a final pixel color.
                for _ in 0..RAY_SAMPLES_PER_PIXEL {
                    let u = (i as f64 + rtweekend::random(0.0..1.0)) / ((img.width() - 1) as f64);
                    let v = (j as f64 + rtweekend::random(0.0..1.0)) / ((img.height() - 1) as f64);
                    let ray = camera.ray(u, v);
                    color = color + ray_color(&ray, &world, RAY_MAX_DEPTH);
                }

                // divide the color by the number of samples
                color / RAY_SAMPLES_PER_PIXEL as f64
            })
            .collect();

        for i in 0..scanline.len() {
            img[j][i] = scanline[i];
        }

        #[cfg(feature = "minifb")]
        {
            // update minifb buffer and render it
            let buffer_offset = (IMAGE_HEIGHT - 1 - j) * IMAGE_WIDTH;
            let buffer_row = &mut buffer[buffer_offset..buffer_offset + IMAGE_WIDTH];
            for i in 0..scanline.len() {
                let rgb8 = color_to_rgb8(&scanline[i]);
                let (r, g, b) = (rgb8[0] as u32, rgb8[1] as u32, rgb8[2] as u32);
                buffer_row[i] = (r << 16) | (g << 8) | b
            }
            window
                .update_with_buffer(&buffer, IMAGE_WIDTH, IMAGE_HEIGHT)
                .unwrap();
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
            let color = img[j][i];
            let rgb8 = color_to_rgb8(&color);
            println!("{} {} {}", rgb8[0], rgb8[1], rgb8[2]);
        }
    }

    Ok(())
}
