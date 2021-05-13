use std::cmp::PartialOrd;

use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::Rng;

/// Convert degrees to radians
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

/// Generate a random number in the range [0,1)
pub fn random<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}

/// Clamp a value so it falls inside the given range
pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
