# Waves

以Rust语言编程的纵波模拟器。

Waves are simulated as equidistant particles connected by linear springs.

## Input

The input files are JSON files in the [`input/`] directory.
Below is a sample of an input file.

The time step size is in seconds ($\mathrm{s}$).

The dimensions are the number of particles along each axis.

The particle distances in meters ($\mathrm{m}$).
A diagonal spring will have a greater length than the listed distances
as given by Pythagorean's Theorem.
A spring along the x-axis will have the length of the first element,
and so forth for springs along the y- and z-axes.

The mass is in kilograms ($\mathrm{kg}$).

The spring constant is in newtons per meter ($\mathrm{N/m}$).

The damping is in newton-seconds per meter ($\mathrm{N \cdot m \cdot s^{-1}}$)
or dimensionally equivalent to kilograms per second ($\mathrm{kg/s}$).

The driving amplitude is in newtons ($\mathrm{N}$).

The driving angular frequency is in radians per second ($\mathrm{rad/s}$).

The driving phase is in radians ($\mathrm{rad}$).

```json
{
    "total_time_steps": 120,
    "time_step_size": 0.5,
    "dimensions": [5, 5, 5],
    "particle_distances": [1.0, 1.0, 1.0],
    "mass": 1.0,
    "spring_constant": 1.0,
    "damping": 1.0,
    "driving": {
        "amplitude": [1.0, 0.0, 0.0],
        "angular_frequency": 1.0,
        "phase": 0.0
    }
}
```

[`input/`]: input/
