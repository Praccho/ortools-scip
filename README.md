# ortools-scip

Rust bindings for the [Google OR-Tools](https://developers.google.com/optimization) SCIP mixed-integer programming solver.

This crate is **fully self-contained** — it automatically downloads pre-built OR-Tools binaries on first build. No manual installation, no environment variables, no setup scripts. Just add the dependency and start solving.

Built for [Brown CS2951O](https://cs.brown.edu/courses/csci2951-o/) students who want to use Rust for optimization assignments, but usable by anyone who needs a MIP solver in Rust.

## Quick start

Add to your `Cargo.toml`:

```toml
[dependencies]
ortools-scip = "0.1"
```

```rust
use ortools_scip::Solver;

fn main() {
    let mut solver = Solver::new("example");

    // Integer variables: x, y in [0, 10]
    let x = solver.make_int_var(0.0, 10.0, "x");
    let y = solver.make_int_var(0.0, 10.0, "y");

    // Constraint: x + 2y <= 14
    let ct = solver.make_constraint(f64::NEG_INFINITY, 14.0);
    ct.set_coefficient(&x, 1.0);
    ct.set_coefficient(&y, 2.0);

    // Maximize 3x + 5y
    solver.objective().set_coefficient(&x, 3.0);
    solver.objective().set_coefficient(&y, 5.0);
    solver.objective().set_maximization();

    let status = solver.solve();
    if status.has_solution() {
        println!("Objective = {}", solver.objective().value());
        println!("x = {}", x.solution_value());
        println!("y = {}", y.solution_value());
    }
}
```

## Features

### Solver

- Create SCIP MIP solver instances
- Solve with default or custom parameters
- Set time limits, thread count, and solver-specific parameters
- Enable/suppress solver output
- Interrupt and resume solving
- Enumerate multiple solutions with `next_solution()`
- Export models in LP or MPS format
- Provide solution hints

### Variables

- Continuous (`make_num_var`), integer (`make_int_var`), and binary (`make_bool_var`)
- Batch creation with `make_int_var_array`, `make_num_var_array`, `make_bool_var_array`
- Modify bounds and integrality after creation
- Set branching priorities
- Read solution values, reduced costs

### Constraints

- Bounded, unbounded, and named variants
- Set and query coefficients
- Modify bounds after creation
- Mark constraints as lazy
- Read dual values

### Objective

- Set coefficients and constant offset
- Maximize or minimize
- Read objective value and best bound after solving

### Solver parameters

- Relative MIP gap tolerance
- Primal/dual tolerance
- Presolve on/off
- LP algorithm (dual simplex, primal simplex, barrier)
- Incrementality and scaling

## How it works

On first `cargo build`, the build script:

1. Downloads the pre-built OR-Tools C++ distribution (~200 MB) for your platform
2. Caches it at `~/.cache/ortools-scip/` so subsequent builds are instant
3. Compiles a thin C++ shim that bridges OR-Tools' C++ API to a C ABI
4. Links everything with rpath so no runtime configuration is needed

### Supported platforms

| Platform | Architecture |
|----------|-------------|
| macOS | arm64 (Apple Silicon), x86_64 |
| Linux | x86_64 (Ubuntu 22.04+), aarch64 |

### System requirements

- A C++ compiler (Xcode Command Line Tools on macOS, `build-essential` on Linux)
- `curl` and `tar` (pre-installed on macOS and virtually all Linux distros)

### Custom OR-Tools installation

If you need a specific OR-Tools version or have it installed elsewhere, set the `ORTOOLS_DIR` environment variable to the root of an extracted OR-Tools C++ distribution:

```sh
export ORTOOLS_DIR=/path/to/or-tools_arm64_macOS-26.2_cpp_v9.15.6755
cargo build
```

## A more complete example

```rust
use ortools_scip::{Solver, SolverParams, DoubleParam};

fn main() {
    let mut solver = Solver::new("knapsack");
    solver.enable_output(); // print solver log
    solver.set_time_limit_ms(30_000); // 30-second time limit

    let weights = [10.0, 20.0, 30.0, 40.0, 50.0];
    let values = [60.0, 100.0, 120.0, 150.0, 200.0];
    let capacity = 80.0;
    let n = weights.len();

    // Binary decision variables: take item i or not
    let items = solver.make_bool_var_array(n, "item");

    // Capacity constraint
    let ct = solver.make_constraint(0.0, capacity);
    for i in 0..n {
        ct.set_coefficient(&items[i], weights[i]);
    }

    // Maximize total value
    for i in 0..n {
        solver.objective().set_coefficient(&items[i], values[i]);
    }
    solver.objective().set_maximization();

    // Solve with custom parameters
    let mut params = SolverParams::new();
    params.set_double(DoubleParam::RelativeMipGap, 0.01); // 1% gap

    let status = solver.solve_with_params(&params);
    println!("Status: {status}");

    if status.has_solution() {
        println!("Total value: {}", solver.objective().value());
        println!("Best bound: {}", solver.objective().best_bound());
        for i in 0..n {
            if items[i].solution_value() > 0.5 {
                println!("  Take item {i} (weight={}, value={})", weights[i], values[i]);
            }
        }
        println!("Wall time: {} ms", solver.wall_time_ms());
        println!("Nodes explored: {}", solver.nodes());
    }
}
```

## License

Apache-2.0
