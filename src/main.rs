use std::io;
use std::io::Write;

mod ppm;
use ppm::Image;

mod vec3;
use vec3::Vec3;

/// RGB color with each channel ranging from 0 to 255
type Color = Vec3<u8>;

fn main() -> io::Result<()> {
    // Image settings
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    // create the image buffer
    let mut img = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, Color::new(0, 0, 0));

    // fill image with test data
    for j in (0..img.height()).rev() {
        eprint!("\r>> Scanlines remaining: {:width$}", j, width = 5);
        io::stdout().flush()?;

        for i in 0..img.width() {
            let r = (i as f64) / ((img.width() - 1) as f64);
            let g = (j as f64) / ((img.height() - 1) as f64);
            let b = 0.25;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;
            img[i][j] = Color::new(ir, ig, ib);
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
            println!("{} {} {}", pix[0], pix[1], pix[2]);
        }
    }

    Ok(())
}
