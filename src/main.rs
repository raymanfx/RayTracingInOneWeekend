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

/// RGB color with each channel ranging from 0.0 to 1.0
type Color = Vec3<f64>;

/// Transform color values from [0.0, 1.0] to [0, 255].
fn write_color(color: &Color) {
    let r = (255.999 * color.x()) as u8;
    let g = (255.999 * color.y()) as u8;
    let b = (255.999 * color.z()) as u8;
    println!("{} {} {}", r, g, b);
}

/// Compute the color of pixel hit by a ray.
fn ray_color(ray: &Ray<f64>, world: &HittableList<f64>) -> Color {
    if let Some(rec) = world.is_hit(ray, 0.0, std::f64::MAX) {
        // assume the normal is a unit length vector in the range [-1.0, 1.0] and map it to the
        // [0.0, 1.0] range since we are going to interpret it as RGB
        return Color::new(
            rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0,
        ) * 0.5;
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
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
    eprintln!(">> Image: {} (W) x {} (H)", IMAGE_WIDTH, IMAGE_HEIGHT);

    // Camera settings
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;
    eprintln!(
        ">> Viewport: {} (W) x {} (H) - focal: {}",
        VIEWPORT_WIDTH, VIEWPORT_HEIGHT, FOCAL_LENGTH
    );

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // "eye" of the scene aka camera center
    let origin = Point3::<f64>::new(0.0, 0.0, 0.0);
    // right handed coordinate system: x axis is horizontal
    let horizontal = Vec3::<f64>::new(VIEWPORT_WIDTH as f64, 0.0, 0.0);
    // right handed coordinate system: y axis is vertical
    let vertical = Vec3::<f64>::new(0.0, VIEWPORT_HEIGHT as f64, 0.0);
    // do the following to get to the lower left corner:
    //  1. go left as far as possible (half the viewport)
    //  2. go down as far as possible (half the viewport)
    //  3. move forward (negative z direction) so we lay flat on the surface
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3::<f64>::new(0.0, 0.0, FOCAL_LENGTH as f64);

    // create the image buffer
    let mut img = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, Color::new(0.0, 0.0, 0.0));

    // fill image with test data
    for j in (0..img.height()).rev() {
        eprint!("\r>> Scanlines remaining: {:width$}", j, width = 5);
        io::stdout().flush()?;

        for i in 0..img.width() {
            let u = (i as f64) / ((img.width() - 1) as f64);
            let v = (j as f64) / ((img.height() - 1) as f64);
            let direction = lower_left_corner + horizontal * u + vertical * v - origin;
            let ray = Ray::new(origin, direction);
            img[j][i] = ray_color(&ray, &world);
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
