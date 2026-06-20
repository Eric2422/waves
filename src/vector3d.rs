//! Module of functions to perform calculations on would be considered 3D
//! vectors in physics, but three-element arrays in Rust

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A 3D vector with x, y, and z values.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector3d(f64, f64, f64);

impl Add for Vector3d {
    type Output = Vector3d;

    /// Adds two [3D vector]s together, returning the [vector] sum.
    /// *Neither* of the original [vector]s are modified.
    ///
    /// [3D vector]: Vector3d
    /// [vector]: Vector3d
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Returns [4, 4, 4].
    /// Vector3d(1, 2, 3) + Vector3d(3, 2, 1)
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Vector3d(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vector3d {
    /// Adds another [3D vector] to this [vector].
    ///
    /// [3D vector]: Vector3d
    /// [vector]: Vector3d
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Neg for Vector3d {
    type Output = Vector3d;

    /// Returns the negative of this [3D vector].
    ///
    /// [3D vector]: Vector3d
    fn neg(self) -> Self::Output {
        Vector3d(-self.0, -self.1, -self.1)
    }
}

impl Sub for Vector3d {
    type Output = Vector3d;

    /// Returns the difference between this [3D vector] and another [3D vector].
    /// *Neither* of the original [vector]s are modified.
    ///
    /// [3D vector]: Vector3d
    /// [vector]: Vector3d
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3d(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vector3d {
    /// Subtract another [3D vector] from this [vector].
    ///
    /// [3D vector]: Vector3d
    /// [vector]: Vector3d
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul<f64> for Vector3d {
    type Output = Vector3d;

    /// Returns the product between this [3D vector] and a scalar.
    ///
    /// [3D vector]: Vector3d
    fn mul(self, rhs: f64) -> Self::Output {
        Vector3d(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vector3d> for f64 {
    type Output = Vector3d;

    /// Returns the product between this scalar and a [3D vector].
    ///
    /// [3D vector]: Vector3d
    fn mul(self, rhs: Vector3d) -> Self::Output {
        Vector3d(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl MulAssign<f64> for Vector3d {
    /// Multiply this [`Vector3d`] by a scalar, *mutating it in the process*.
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Mul<Vector3d> for Vector3d {
    type Output = f64;

    /// Return the dot product of this [3D vector] and another [3D vector].
    ///
    /// [3D vector]: Vector3d
    fn mul(self, rhs: Vector3d) -> Self::Output {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

impl Div<f64> for Vector3d {
    type Output = Vector3d;

    /// Return the result of this [3D vector] divided by a scalar, without
    /// mutating the [3D vector].
    ///
    /// [3D vector]: Vector3d
    fn div(self, rhs: f64) -> Self::Output {
        Vector3d(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<f64> for Vector3d {
    /// Divide this [3D vector] by a scalar, *mutating it in the process*.
    /// 
    /// [3D vector]: Vector3d
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

/// Add two 3D vectors together, returning the vector sum.
/// *Neither* of the original arrays are modified.
///
/// # Examples
///
/// ```rust
/// // Returns [4, 4, 4].
/// add_arrays([1, 2, 3], [3, 2, 1])
/// ```
pub fn add_arrays(array1: [f64; 3], array2: [f64; 3]) -> [f64; 3] {
    [
        array1[0] + array2[0],
        array1[1] + array2[1],
        array1[2] + array2[2],
    ]
}

/// Multiply a 3D vector by a scalar value, returning the product.
/// The original array is *not* modified.
///
/// # Examples
///
/// ```rust
/// // Returns [10, 20, 30].
/// multiply_array_by_scalar([1, 2, 3], 10)
/// ```
pub fn multiply_array_by_scalar(array: [f64; 3], scalar: f64) -> [f64; 3] {
    [array[0] * scalar, array[1] * scalar, array[2] * scalar]
}

/// Divide a 3D vector by a scalar value, returning the dividend. The original
/// array is *not* modified.
///
/// It is prefereable to use this function rather than use
/// [`multiply_array_by_scalar()`] with the scalar inverse due to floating point
/// error.
///
/// # Examples
/// ```rust
/// // Returns approximately [2, 4, 6]
/// divide_array_by_scalar([10, 20, 30], 5)
/// ```
pub fn divide_array_by_scalar(array: [f64; 3], scalar: f64) -> [f64; 3] {
    [array[0] / scalar, array[1] / scalar, array[2] / scalar]
}

/// Calculate the magnitude of the given 3D vector.
///
/// # Examples
///
/// ```rust
/// // Returns approximately 5.0.
/// calculate_array_magnitude([3, 4, 0])
/// ```
pub fn calculate_array_magnitude(array: [f64; 3]) -> f64 {
    (array[0].powi(2) + array[1].powi(2) + array[2].powi(2)).sqrt()
}
