# Euler_IVP
This project provides implementations of the Euler method for solving ordinary differential equations (ODEs) in Rust (`euler.rs`) and Python (`Euler.py`).

## Function: `euler`

Solves an initial value problem for an ODE using the explicit Euler method.

**Parameters:**

- **f:** A function defining the ODE, `f(t, y)`
- **initial_value:** Initial value of the dependent variable
- **start_time:** Starting time for the solution
- **end_time:** Ending time for the solution
- **n_steps:** Number of steps between start and end times

**Returns:**  
Array (or list) of solution values at each step.

## Example Usage

euler(f, 1.0, 0.0, 5.0, 20)
