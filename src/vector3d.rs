//! Module of functions to perform calculations on would be considered 3D
//! vectors in physics, but three-element arrays in Rust

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use uom;

/// A 3D vector with x, y, and z values.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Either converts a three-element [`f64`] array into a [3D vector]
/// or packs three [`f64`] floats into a [3D vector].
///
/// [3D vector]: Vector3d
#[macro_export]
macro_rules! vector3d {
    ($array:expr) => {
        Vector3d {
            x: $array[0],
            y: $array[1],
            z: $array[2],
        }
    };

    ($x:expr, $y:expr, $z:expr) => {
        Vector3d {
            x: $x,
            y: $y,
            z: $z,
        }
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
    /// // Ignoring possible floating point errors, returns Vector3d {x: 4.0, y: 4.0, z: 4.0}.
    /// Vector3d {
    ///     x: 1.0,
    ///     y: 2.0,
    ///     z: 3.0,
    /// } + Vector3d {
    ///     x: 3.0,
    ///     y: 2.0,
    ///     z: 1.0,
    /// }
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3d {
    /// Adds another [3D vector] to this [vector].
    ///
    /// [3D vector]: Vector3d
    /// [vector]: Vector3d
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Neg for Vector3d {
    type Output = Vector3d;

    /// Returns the negative of this [3D vector].
    ///
    /// [3D vector]: Vector3d
    fn neg(self) -> Self::Output {
        Vector3d {
            x: -self.x,
            y: -self.y,
            z: -self.y,
        }
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
        Vector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3d {
    /// Subtract another [3D vector] from this [vector].
    ///
    /// [3D vector]: Vector3d
    /// [vector]: Vector3d
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Vector3d {
    type Output = Vector3d;

    /// Returns the product between this [3D vector] and a scalar.
    ///
    /// ```rust
    /// // Returns Vector3d {x: 10.0, y: 20.0, z: 30.0}.
    /// Vector3d {
    ///     x: 1.0,
    ///     y: 2.0,
    ///     z: 3.0,
    /// } * 10.0
    /// ```
    /// [3D vector]: Vector3d
    fn mul(self, rhs: f64) -> Self::Output {
        Vector3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3d> for f64 {
    type Output = Vector3d;

    /// Returns the product between this scalar and a [3D vector].
    ///
    /// [3D vector]: Vector3d
    fn mul(self, rhs: Vector3d) -> Self::Output {
        Vector3d {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vector3d {
    /// Multiply this [`Vector3d`] by a scalar, *mutating it in the process*.
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<Vector3d> for Vector3d {
    type Output = f64;

    /// Return the dot product of this [3D vector] and another [3D vector].
    ///
    /// [3D vector]: Vector3d
    fn mul(self, rhs: Vector3d) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
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
    /// Vector3d {
    ///     x: 10,
    ///     y: 20,
    ///     z: 30,
    /// } / 5
    /// ```
    fn div(self, rhs: f64) -> Self::Output {
        Vector3d {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vector3d {
    /// Divide this [3D vector] by a scalar, *mutating it in the process*.
    ///
    /// [3D vector]: Vector3d
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Display for Vector3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vector3d {
    /// Return the zero vector, i.e., `Vector3d {x: 0.0, y: 0.0, z: 0.0}`.
    pub fn zero() -> Vector3d {
        Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Calculate the magnitude of this [3D vector].
    ///
    /// [3D vector]: Vector3d
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Returns approximately 5.0.
    /// Vector3d { x: 3, y: 4, z: 0 }
    /// ```
    pub fn get_magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Return the normalized unit vector of this [3D vector].
    ///
    /// [3D vector]: Vector3d
    pub fn get_normalized(&self) -> Vector3d {
        *self / self.get_magnitude()
    }
}
