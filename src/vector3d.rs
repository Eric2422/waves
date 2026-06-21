//! Module of functions to perform calculations on would be considered 3D
//! vectors in physics, but three-element arrays in Rust

use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    sync::mpsc::RecvTimeoutError,
};


/// A 3D vector with x, y, and z values.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector3d(pub f64, pub f64, pub f64);

/// Unpacks a three-element [`f64`] array into a [`Vector3d`].
#[macro_export]
macro_rules! vector_3d {
    ($array:expr) => {
        Vector3d($array[0], $array[1], $array[2])
    };
}

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
    /// // Ignoring possible floating point errors, returns Vector3d(4.0, 4.0, 4.0).
    /// Vector3d(1.0, 2.0, 3.0) + Vector3d(3.0, 2.0, 1.0)
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
    /// ```rust
    /// // Returns Vector3d(10.0, 20.0, 30.0).
    /// Vector3d(1.0, 2.0, 3.0) * 10.0
    /// ```
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
    /// Has less floating point error than [multiplying] by the inverse.
    ///
    /// [3D vector]: Vector3d
    /// [multiplying]: Vector3d::mul
    ///
    /// # Examples
    /// ```rust
    /// // Returns approximately [2, 4, 6]
    /// Vector3d(10, 20, 30) / 5
    /// ```
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

impl Vector3d {
    /// Return the zero vector, i.e., `Vector3d(0.0, 0.0, 0.0)`.
    pub fn zero() -> Vector3d {
        Vector3d(0.0, 0.0, 0.0)
    }

    /// Calculate the magnitude of this [3D vector].
    ///
    /// [3D vector]: Vector3d
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Returns approximately 5.0.
    /// Vector3d(3, 4, 0)
    /// ```
    pub fn calculate_magnitude(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
}
