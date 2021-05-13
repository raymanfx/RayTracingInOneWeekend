use std::io;
use std::io::Write;

mod ppm;
use ppm::Image;

mod vec3;
use vec3::Vec3;

/// RGB color with each channel ranging from 0.0 to 1.0
type Color = Vec3<f64>;

/// Transform color values from [0.0, 1.0] to [0, 255].
fn write_color(color: &Color) {
    let r = (255.999 * color.x()) as u8;
    let g = (255.999 * color.y()) as u8;
    let b = (255.999 * color.z()) as u8;
    println!("{} {} {}", r, g, b);
}

fn main() -> io::Result<()> {
    // Image settings
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    // create the image buffer
    let mut img = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, Color::new(0.0, 0.0, 0.0));

    // fill image with test data
    for j in (0..img.height()).rev() {
        eprint!("\r>> Scanlines remaining: {:width$}", j, width = 5);
        io::stdout().flush()?;

        for i in 0..img.width() {
            let r = (i as f64) / ((img.width() - 1) as f64);
            let g = (j as f64) / ((img.height() - 1) as f64);
            let b = 0.25;
            img[i][j] = Color::new(r, g, b);
        }
    }
    eprintln!("\n>> Render done");

    // print PPM header
    println!("P3");
    println!("{} {}", img.width(), img.height());
    println!("255");
    // print PPM data
    for j in 0..img.height() {
        for i in 0..img.width() {
            let pix = img[j][i];
            write_color(&pix);
        }
    }

    Ok(())
}
