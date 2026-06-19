
/// Add two 3D vectors together, returning the vector sum.
/// *Neither* of the original arrays are modified.
///
/// # Examples
///
/// ```rust
/// // Returns [4, 4, 4].
/// let array_sum = add_3d_vectors([1, 2, 3], [3, 2, 1]);
/// ```
pub fn add_vectors(vector1: [f64; 3], vector2: [f64; 3]) -> [f64; 3] {
    [
        vector1[0] + vector2[0],
        vector1[1] + vector2[1],
        vector1[2] + vector2[2],
    ]
}

/// Multiply a 3D vector by a scalar value, returning the product.
/// The original array is *not* modified.
///
/// # Examples
///
/// ```rust
/// // Returns [10, 20, 30].
/// let new_array = multiply_array_scalar([1, 2, 3], 10);
/// ```
pub fn multiply_vector_by_scalar(vector: [f64; 3], scalar: f64) -> [f64; 3] {
    [vector[0] * scalar, vector[1] * scalar, vector[2] * scalar]
}

/// Calculate the magnitude of the given 3D vector.
pub fn calculate_vector_magnitude(vector: [f64; 3]) -> f64 {
    (vector[0].powi(2) + vector[1].powi(2) + vector[2].powi(2)).sqrt()
}