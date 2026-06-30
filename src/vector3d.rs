//! Module of functions to perform calculations on would be considered 3D
//! vectors in physics, but three-element arrays in Rust

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use uom::{
    self,
    si::{Dimension, Quantity, SI},
};

/// A 3D vector with x, y, and z values.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vector3d<D: Dimension>
where
    D: Dimension,
    <D as Dimension>::Kind: uom::Kind,
{
    pub x: Quantity<D, SI<f64>, f64>,
    pub y: Quantity<D, SI<f64>, f64>,
    pub z: Quantity<D, SI<f64>, f64>,
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

impl<D> Add for Vector3d<D>
where
    D: Dimension,
    <D as Dimension>::Kind: uom::Kind,
{
    type Output = Vector3d<D>;

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

impl<D> AddAssign for Vector3d<D>
where
    D: Dimension,
    <D as Dimension>::Kind: uom::Kind,
{
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

impl<D> Neg for Vector3d<D>
where
    D: Dimension,
    <D as Dimension>::Kind: uom::Kind,
{
    type Output = Vector3d<D>;

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

impl<D> Sub for Vector3d<D>
where
    D: Dimension,
    <D as Dimension>::Kind: uom::Kind,
{
    type Output = Vector3d<D>;

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

impl<D> SubAssign for Vector3d<D>
where
    D: Dimension,
    <D as Dimension>::Kind: uom::Kind,
{
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

impl<D> Mul<f64> for Vector3d<D>
where
    D: Dimension,
    <D as Dimension>::Kind: uom::Kind,
{
    type Output = Vector3d<D>;

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

impl<D> Mul<Vector3d<D>> for Quantity<D, uom::si::SI<f64>, f64>
where
    D: Dimension,
    <D as Dimension>::Kind: uom::Kind,
{
    type Output = Vector3d<D>;

    /// Returns the product between this scalar and a [3D vector].
    ///
    /// [3D vector]: Vector3d
    fn mul(self, rhs: Vector3d<D>) -> Self::Output {
        Vector3d {
            x: self * rhs.x.value,
            y: self * rhs.y.value,
            z: self * rhs.z.value,
        }
    }
}

impl<D> MulAssign<f64> for Vector3d<D>
where
    D: Dimension,
    <D as Dimension>::Kind: uom::Kind,
{
    /// Multiply this [`Vector3d`] by a scalar, *mutating it in the process*.
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<D> Div<f64> for Vector3d<D>
where
    D: Dimension,
    <D as Dimension>::Kind: uom::Kind,
{
    type Output = Vector3d<D>;

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

impl<D> DivAssign<f64> for Vector3d<D>
where
    D: Dimension,
    <D as Dimension>::Kind: uom::Kind,
{
    /// Divide this [3D vector] by a scalar, *mutating it in the process*.
    ///
    /// [3D vector]: Vector3d
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<D> Display for Vector3d<D>
where
    D: Dimension,
    <D as Dimension>::Kind: uom::Kind,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }
}

impl<D> Vector3d<D>
where
    D: Dimension + Sized + uom::ConstZero,
    <D as Dimension>::Kind: uom::Kind,
    <D as Dimension>::Kind: Sized,
{
    /// Return the zero vector, i.e., `Vector3d {x: 0.0, y: 0.0, z: 0.0}`.
    pub fn zero() -> Vector3d<D> {
        Vector3d {
            // The default value should 0.0, but is highly fragile.
            // TODO: Replace with something more stable.
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
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
    pub fn get_magnitude(&self) -> D {
        (self.x.powi(uom::typenum::P2::new())
            + self.y.powi(uom::typenum::P2::new())
            + self.z.powi(uom::typenum::P2::new()))
        .sqrt()
    }

    /// Return the normalized unit vector of this [3D vector].
    ///
    /// [3D vector]: Vector3d
    pub fn get_normalized(&self) -> Vector3d<D> {
        *self / self.get_magnitude()
    }
}
