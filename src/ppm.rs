use std::ops::{Index, IndexMut};

/// A simple PPM image struct.
///
/// Each pixel is represented by a RGB triplet.
/// We keep the implementation generic at this point, but most of the time you
/// will want to use T = [u8; 3] for ASCII color channels at 8 bits.
pub struct Image<T> {
    width: usize,
    height: usize,
    pixels: Vec<T>,
}

impl<T: Clone> Image<T> {
    /// Create a new PPM image.
    ///
    /// * `width` - Width in pixels.
    /// * `height` - Height in pixels.
    /// * `value` - Initial pixel value.
    pub fn new(width: usize, height: usize, value: T) -> Self {
        Image {
            width,
            height,
            pixels: vec![value; height * width],
        }
    }

    /// Returns the width in pixels.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height in pixels.
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T> Index<usize> for Image<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let offset = index * self.width;
        &self.pixels[offset..offset + self.width]
    }
}

impl<T> IndexMut<usize> for Image<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let offset = index * self.width;
        &mut self.pixels[offset..offset + self.width]
    }
}
