use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

/// A simple vector in 3D space.
#[derive(Debug, Clone, Copy)]
pub struct Vec<T: Copy, const N: usize>(pub [T; N]);

impl<T: Copy, const N: usize> Index<usize> for Vec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Copy, const N: usize> IndexMut<usize> for Vec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

// -Vector

impl<T: Copy, const N: usize> Neg for Vec<T, N>
where
    T: Neg<Output = T>,
{
    type Output = Vec<T, N>;

    fn neg(mut self) -> Vec<T, N> {
        for i in 0..N {
            self[i] = -self[i];
        }
        self
    }
}

// Vector + Vector

impl<T: Copy, const N: usize> Add for Vec<T, N>
where
    T: Add<Output = T>,
{
    type Output = Vec<T, N>;

    fn add(mut self, other: Vec<T, N>) -> Vec<T, N> {
        for i in 0..N {
            self[i] = self[i] + other[i];
        }
        self
    }
}

// Vector + Scalar

impl<T: Copy, const N: usize> Add<T> for Vec<T, N>
where
    T: Add<Output = T>,
{
    type Output = Vec<T, N>;

    fn add(mut self, scalar: T) -> Vec<T, N> {
        for i in 0..N {
            self[i] = self[i] + scalar;
        }
        self
    }
}

// Vector - Vector

impl<T: Copy, const N: usize> Sub for Vec<T, N>
where
    T: Sub<Output = T>,
{
    type Output = Vec<T, N>;

    fn sub(mut self, other: Vec<T, N>) -> Vec<T, N> {
        for i in 0..N {
            self[i] = self[i] - other[i];
        }
        self
    }
}

// Vector - Scalar

impl<T: Copy, const N: usize> Sub<T> for Vec<T, N>
where
    T: Sub<Output = T>,
{
    type Output = Vec<T, N>;

    fn sub(mut self, scalar: T) -> Vec<T, N> {
        for i in 0..N {
            self[i] = self[i] - scalar;
        }
        self
    }
}

// Vector * Scalar

impl<T: Copy, const N: usize> Mul<T> for Vec<T, N>
where
    T: Mul<Output = T>,
{
    type Output = Vec<T, N>;

    fn mul(mut self, scalar: T) -> Vec<T, N> {
        for i in 0..N {
            self[i] = self[i] * scalar;
        }
        self
    }
}

// Vector / Scalar

impl<T: Copy, const N: usize> Div<T> for Vec<T, N>
where
    T: Div<Output = T>,
{
    type Output = Vec<T, N>;

    fn div(mut self, scalar: T) -> Vec<T, N> {
        for i in 0..N {
            self[i] = self[i] / scalar;
        }
        self
    }
}

// Convenience helpers

impl<T: Copy, const N: usize> Vec<T, N>
where
    T: Add<Output = T> + Mul<Output = T> + Into<f64>,
{
    /// Returns the squared length.
    pub fn length_squared(&self) -> T {
        let mut length = self[0] * self[0];
        for i in 1..N {
            length = length + self[i] * self[i];
        }
        length
    }

    /// Returns the length.
    pub fn length(&self) -> f64 {
        self.length_squared().into().sqrt()
    }
}

impl<const N: usize> Vec<f32, N> {
    /// Returns the unit length.
    pub fn normalized(&self) -> Vec<f32, N> {
        *self / self.length() as f32
    }
}

impl<const N: usize> Vec<f64, N> {
    /// Returns the unit length.
    pub fn normalized(&self) -> Vec<f64, N> {
        *self / self.length()
    }
}

// Utility functions

impl<T: Copy, const N: usize> Vec<T, N>
where
    T: Add<Output = T> + Mul<Output = T>,
{
    /// Computes the dot product between two vectors.
    ///
    /// * `u` - First vector.
    /// * `v` - Second vector.
    pub fn dot(u: &Vec<T, N>, v: &Vec<T, N>) -> T {
        let mut sum = u[0] * v[0];
        for i in 1..N {
            sum = sum + u[i] * v[i];
        }
        sum
    }
}

pub type Vec3<T> = Vec<T, 3>;

impl<T: Copy> Vec3<T> {
    /// Returns the X direction.
    pub fn x(&self) -> T {
        self[0]
    }

    /// Returns the Y direction.
    pub fn y(&self) -> T {
        self[1]
    }

    /// Returns the Z direction.
    pub fn z(&self) -> T {
        self[2]
    }
}

impl<T: Copy> Vec3<T> {
    /// Create a new 3D vector.
    ///
    /// * `x` - X direction.
    /// * `y` - Y direction.
    /// * `z` - Z direction.
    pub fn new3(x: T, y: T, z: T) -> Vec<T, 3> {
        Vec { 0: [x, y, z] }
    }
}

impl<T: Copy> Vec3<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    /// Computes the cross product between two vectors.
    ///
    /// * `u` - First vector.
    /// * `v` - Second vector.
    pub fn cross(u: &Vec<T, 3>, v: &Vec<T, 3>) -> Vec<T, 3> {
        Vec {
            0: [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ],
        }
    }
}
