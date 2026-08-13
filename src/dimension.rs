//! Dimension aliases for physical dimensions from [`uom`].

use uom::si::f64::{MassRate, SurfaceTension};


/// Alias for [`SurfaceTension`] to more accurately describe spring stiffness
/// rather than surface tension, which are dimensionally equivalent.
pub type SpringStiffness = SurfaceTension;
/// Alias for [`MassRate`]
/// because the damping coefficient and rate of mass change
/// are dimensionally equivalent,
/// e.g., newton-seconds per metre (N⋅s⋅m⁻¹) or kilograms per second (kg/s).
pub type ViscousDamping = MassRate;
