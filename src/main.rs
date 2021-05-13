mod ppm;

fn main() {
    // Image settings
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    // create the image buffer
    let mut img = ppm::Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, [0u8; 3]);
}
