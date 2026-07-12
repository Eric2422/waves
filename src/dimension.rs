//! Dimension aliases for physical dimensions from [`uom`].

use uom::si::f64::{MassRate, SurfaceTension};


/// Alias for [`SurfaceTension`] to more accurately describe spring constants
/// rather than surface tension, which are dimensionally equivalent.
pub type SpringConstant = SurfaceTension;
/// Alias for [`MassRate`]
/// because the damping coefficient and rate of mass change
/// are dimensionally equivalent,
/// e.g., newton-seconds per meter (N⋅s⋅m⁻¹)or kilograms per second (kg/s).
pub type ViscousDamping = MassRate;
