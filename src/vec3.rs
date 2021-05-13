use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

/// A simple vector in 3D space.
#[derive(Debug, Clone, Copy)]
pub struct Vec3<T: Copy> {
    e: [T; 3],
}

impl<T: Copy> Vec3<T> {
    /// Create a new vector.
    ///
    /// * `x` - X direction.
    /// * `y` - Y direction.
    /// * `z` - Z direction.
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { e: [x, y, z] }
    }
}

impl<T: Copy> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl<T: Copy> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

// Vector + Vector

impl<T: Copy> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

// Vector + Scalar

impl<T: Copy> Add<T> for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Vec3<T>;

    fn add(self, scalar: T) -> Vec3<T> {
        Vec3 {
            e: [self[0] + scalar, self[1] + scalar, self[2] + scalar],
        }
    }
}

// Vector - Vector

impl<T: Copy> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Vec3<T>;

    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        }
    }
}

// Vector - Scalar

impl<T: Copy> Sub<T> for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Vec3<T>;

    fn sub(self, scalar: T) -> Vec3<T> {
        Vec3 {
            e: [self[0] - scalar, self[1] - scalar, self[2] - scalar],
        }
    }
}

// Vector * Scalar

impl<T: Copy> Mul<T> for Vec3<T>
where
    T: Mul<Output = T>,
{
    type Output = Vec3<T>;

    fn mul(self, scalar: T) -> Vec3<T> {
        Vec3 {
            e: [self[0] * scalar, self[1] * scalar, self[2] * scalar],
        }
    }
}

// Vector / Scalar

impl<T: Copy> Div<T> for Vec3<T>
where
    T: Div<Output = T>,
{
    type Output = Vec3<T>;

    fn div(self, scalar: T) -> Vec3<T> {
        Vec3 {
            e: [self[0] / scalar, self[1] / scalar, self[2] / scalar],
        }
    }
}

// Convenience helpers

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

impl<T: Copy> Vec3<T>
where
    T: Add<Output = T> + Mul<Output = T> + Into<f64>,
{
    /// Returns the squared length.
    pub fn length_squared(&self) -> T {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    /// Returns the length.
    pub fn length(&self) -> f64 {
        self.length_squared().into().sqrt()
    }
}

impl Vec3<f32> {
    /// Returns the unit length.
    pub fn normalized(&self) -> Vec3<f32> {
        *self / self.length() as f32
    }
}

impl Vec3<f64> {
    /// Returns the unit length.
    pub fn normalized(&self) -> Vec3<f64> {
        *self / self.length()
    }
}

// Utility functions

impl<T: Copy> Vec3<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Into<f64>,
{
    /// Computes the dot product between two vectors.
    ///
    /// * `u` - First vector.
    /// * `v` - Second vector.
    pub fn dot(u: &Vec3<T>, v: &Vec3<T>) -> T {
        u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
    }

    /// Computes the cross product between two vectors.
    ///
    /// * `u` - First vector.
    /// * `v` - Second vector.
    pub fn cross(u: &Vec3<T>, v: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            e: [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ],
        }
    }
}
