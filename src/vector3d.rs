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
pub struct Vector3d<Q> {
    pub x: Q,
    pub y: Q,
    pub z: Q,
}

/// Either converts a three-element [`f64`] array into a [3D vector]
/// or packs three [`f64`] floats into a [3D vector].
///
/// [3D vector]: Vector3d
///
/// # Examples
///
/// ```rust
/// assert_eq!(Vector3d {0, 1, 2}, vector3d!([1, 2, 3]));
/// assert_eq!(Vector3d {0, 1, 2}, vector3d!(1, 2, 3));
/// ```
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

impl<Q: std::ops::Add<Output = Q>> Add for Vector3d<Q> {
    type Output = Vector3d<Q>;

    /// Adds two [3D vector]s together, returning the [vector] sum.
    /// *Neither* of the original [vector]s are modified.
    ///
    /// [3D vector]: Vector3d
    /// [vector]: Vector3d
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     Vector3d {
    ///         x: 4.0,
    ///         y: 4.0,
    ///         z: 4.0
    ///     },
    ///     Vector3d {
    ///         x: 1.0,
    ///         y: 2.0,
    ///         z: 3.0
    ///     } + Vector3d {
    ///         x: 3.0,
    ///         y: 2.0,
    ///         z: 1.0
    ///     }
    /// )
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<Q: std::ops::AddAssign> AddAssign for Vector3d<Q> {
    /// Adds another [3D vector] to this [vector].
    ///
    /// [3D vector]: Vector3d
    /// [vector]: Vector3d
    ///
    /// # Examples
    ///
    /// ```rust
    /// let example_vector = Vector3d {
    ///     x: 2.0,
    ///     y: 4.0,
    ///     z: 6.0,
    /// };
    /// example_vector += Vector3d {
    ///     x: 1.0,
    ///     y: 2.0,
    ///     z: 3.0,
    /// };
    ///
    /// assert_eq!(
    ///     Vector3d {
    ///         x: 3.0,
    ///         y: 5.0,
    ///         z: 9.0
    ///     },
    ///     example_vector
    /// );
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<Q: std::ops::Neg<Output = Q>> Neg for Vector3d<Q> {
    type Output = Vector3d<Q>;

    /// Returns the negative of this [3D vector].
    ///
    /// [3D vector]: Vector3d
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     Vector3d {
    ///         x: -1.0,
    ///         y: -1.0,
    ///         z: -1.0,
    ///     },
    ///     -Vector3d {
    ///         x: 1.0,
    ///         y: 1.0,
    ///         z: 1.0,
    ///     }
    /// )
    /// ```
    fn neg(self) -> Self::Output {
        Vector3d {
            x: -self.x,
            y: -self.y,
            z: -self.y,
        }
    }
}

impl<Q: std::ops::Sub<Output = Q>> Sub for Vector3d<Q> {
    type Output = Vector3d<Q>;

    /// Returns the difference between this [3D vector] and another [3D vector].
    /// *Neither* of the original [vector]s are modified.
    ///
    /// [3D vector]: Vector3d
    /// [vector]: Vector3d
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     Vector3d {
    ///         x: 1.0,
    ///         y: 2.0,
    ///         z: 3.0
    ///     },
    ///     Vector3d {
    ///         x: 4.0,
    ///         y: 4.0,
    ///         z: 4.0
    ///     } - Vector3d {
    ///         x: 3.0,
    ///         y: 2.0,
    ///         z: 1.0
    ///     }
    /// )
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<Q: std::ops::SubAssign> SubAssign for Vector3d<Q> {
    /// Subtract another [3D vector] from this [vector].
    ///
    /// [3D vector]: Vector3d
    /// [vector]: Vector3d
    ///
    /// # Examples
    ///
    /// ```rust
    /// let example_vector = Vector3d {
    ///     x: 3.0,
    ///     y: 5.0,
    ///     z: 9.0,
    /// };
    /// example_vector -= Vector3d {
    ///     x: 1.0,
    ///     y: 2.0,
    ///     z: 3.0,
    /// };
    ///
    /// assert_eq!(
    ///     Vector3d {
    ///         x: 2.0,
    ///         y: 4.0,
    ///         z: 6.0,
    ///     },
    ///     example_vector
    /// );
    /// ```
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<Q: std::ops::Mul<f64, Output = Q>> Mul<f64> for Vector3d<Q> {
    type Output = Vector3d<Q>;

    /// Returns the product between this [3D vector] and a scalar.
    ///
    /// [3D vector]: Vector3d
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     Vector3d {
    ///         x: 10.0,
    ///         y: 20.0,
    ///         z: 30.0
    ///     },
    ///     Vector3d {
    ///         x: 1.0,
    ///         y: 2.0,
    ///         z: 3.0,
    ///     } * 10.0
    /// )
    /// ```
    fn mul(self, rhs: f64) -> Self::Output {
        Vector3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<Q> Mul<Vector3d<Q>> for Q {
    type Output = Vector3d<Q>;

    /// Returns the product between this scalar and a [3D vector].
    ///
    /// [3D vector]: Vector3d
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     Vector3d {
    ///         x: 10.0,
    ///         y: 20.0,
    ///         z: 30.0
    ///     },
    ///     10.0 * Vector3d {
    ///         x: 1.0,
    ///         y: 2.0,
    ///         z: 3.0,
    ///     }
    /// )
    /// ```
    fn mul(self, rhs: Vector3d<Q>) -> Self::Output {
        Vector3d {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl<Q: std::ops::MulAssign<f64>> MulAssign<f64> for Vector3d<Q> {
    /// Multiply this [`Vector3d`] by a scalar, *mutating it in the process*.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let example_vector = Vector3d {
    ///     x: 1.0,
    ///     y: 2.0,
    ///     z: 3.0,
    /// };
    /// example_vector *= 10.0;
    /// assert_eq!(
    ///     Vector3d {
    ///         x: 10.0,
    ///         y: 20.0,
    ///         z: 30.0,
    ///     },
    ///     example_vector
    /// );
    /// ```
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<Q: std::ops::Div<f64, Output = Q>> Div<f64> for Vector3d<Q> {
    type Output = Vector3d<Q>;

    /// Return the result of this [3D vector] divided by a scalar, without
    /// mutating the [3D vector].
    ///
    /// Has less floating point error than [multiplying] by the inverse.
    ///
    /// [3D vector]: Vector3d
    /// [multiplying]: Vector3d::mul
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     Vector3d {
    ///         x: 2.0,
    ///         y: 4.0,
    ///         z: 6.0
    ///     },
    ///     Vector3d {
    ///         x: 10.0,
    ///         y: 20.0,
    ///         z: 30.0,
    ///     } / 5.0
    /// )
    /// ```
    fn div(self, rhs: f64) -> Self::Output {
        Vector3d {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<Q: std::ops::DivAssign<f64>> DivAssign<f64> for Vector3d<Q> {
    /// Divide this [3D vector] by a scalar, *mutating it in the process*.
    ///
    /// [3D vector]: Vector3d
    ///
    /// # Examples
    ///
    /// ```rust
    /// let dividend_array = Vector3d {
    ///     x: 10.0,
    ///     y: 20.0,
    ///     z: 30.0,
    /// };
    /// dividend_array /= 5.0;
    /// assert_eq!(
    ///     Vector3d {
    ///         x: 2.0,
    ///         y: 4.0,
    ///         z: 6.0
    ///     },
    ///     dividend_array
    /// )
    /// ```
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<Q: std::fmt::Debug> Display for Vector3d<Q> {
    /// Prints out the x, y, and z values in tuple vector notation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     "(1.0, 2.0, 3.0)",
    ///     (Vector3d {
    ///         x: 1.0,
    ///         y: 2.0,
    ///         z: 3.0
    ///     })
    ///     .to_string()
    ///     .unwrap()
    /// );
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }
}

impl<Q> Vector3d<Q>
where
    Q: Dimension + Sized + uom::ConstZero,
    <D as Dimension>::Kind: uom::Kind,
    <D as Dimension>::Kind: Sized,
{
    /// Return the zero vector, i.e., `Vector3d {x: 0.0, y: 0.0, z: 0.0}`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     Vector3d {
    ///         x: 0.0,
    ///         y: 0.0,
    ///         z: 0.0
    ///     },
    ///     Vector3d::zero()
    /// );
    /// ```
    pub fn zero() -> Vector3d<Q> {
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
    /// assert_eq!(
    ///     3.0,
    ///     (Vector3d {
    ///         x: 1.0,
    ///         y: 2.0,
    ///         z: 2.0,
    ///     })
    ///     .get_magnitude()
    /// );
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     Vector3d {
    ///         x: 0.33,
    ///         y: 0.67,
    ///         z: 0.67
    ///     },
    ///     (Vector3d {
    ///         x: 1.0,
    ///         y: 2.0,
    ///         z: 2.0
    ///     })
    ///     .get_normalized()
    /// );
    /// ```
    pub fn get_normalized(&self) -> Vector3d<Q> {
        *self / self.get_magnitude()
    }
}
