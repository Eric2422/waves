# Waves

A wave simulator programmed in Rust.

Waves are simulated as equidistant particles connected by linear springs.

## Input

The input files are JSON files in the [`input/`] directory.
Below is a sample of an input file.

The time step size is in seconds (s).

The mass is in kilograms (kg).

The springs lengths are in meters (m).
It describes the distance between particles,
so a diagonal spring will have a greater length than reported.
A spring along the x-axis will have the length of the first element,
and so forth for springs along the y- and z-axes.

The spring constant is in newtons per meter (N/m).

The damping is in newton-seconds per meter (N⋅m⋅s⁻¹)
or dimensionally equivalent to kilograms per second (kg/s).

The driving amplitude is in newtons (N).

The driving angular frequency is in radians per second (rad/s).

The driving phase is in radians (rad).

```json
{
    "total_time_steps": 120,
    "time_step_size": 0.5,
    "dimensions": [5, 5, 5],
    "mass": 1.0,
    "spring_lengths": [1.0, 1.0, 1.0],
    "spring_constant": 1.0,
    "damping": 1.0,
    "driving_amplitude": [1.0, 0.0, 0.0],
    "driving_angular_frequency": 1.0,
    "driving_phase": 0.0
}
```

[`input/`]: input/
