//! Module of functions to perform calculations on would be considered 3D
//! vectors in physics, but three-element arrays in Rust

/// Add two 3D vectors together, returning the vector sum.
/// *Neither* of the original arrays are modified.
///
/// # Examples
///
/// ```rust
/// // Returns [4, 4, 4].
/// let array_sum = add_arrays([1, 2, 3], [3, 2, 1]);
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
/// let new_array = multiply_array_by_scalar([1, 2, 3], 10);
/// ```
pub fn multiply_array_by_scalar(array: [f64; 3], scalar: f64) -> [f64; 3] {
    [array[0] * scalar, array[1] * scalar, array[2] * scalar]
}

/// Calculate the magnitude of the given 3D vector.
///
/// # Examples
///
/// ```rust
/// // Returns approximately 5.0.
/// let magnitude = calculate_array_magnitude([3, 4, 0]);
/// ```
pub fn calculate_array_magnitude(array: [f64; 3]) -> f64 {
    (array[0].powi(2) + array[1].powi(2) + array[2].powi(2)).sqrt()
}
