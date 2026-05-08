use ortools_scip::Solver;

fn main() {
    let mut solver = Solver::new("basic_example");
    println!("Solver version: {}", solver.version());

    // Variables: x in [0, 10], y in [0, 10] (integer)
    let x = solver.make_int_var(0.0, 10.0, "x");
    let y = solver.make_int_var(0.0, 10.0, "y");

    // Constraint: x + 2y <= 14
    let ct = solver.make_constraint(f64::NEG_INFINITY, 14.0);
    ct.set_coefficient(&x, 1.0);
    ct.set_coefficient(&y, 2.0);

    // Constraint: 3x + y <= 14
    let ct2 = solver.make_constraint(f64::NEG_INFINITY, 14.0);
    ct2.set_coefficient(&x, 3.0);
    ct2.set_coefficient(&y, 1.0);

    // Maximize 3x + 5y
    solver.objective().set_coefficient(&x, 3.0);
    solver.objective().set_coefficient(&y, 5.0);
    solver.objective().set_maximization();

    println!("Number of variables: {}", solver.num_variables());
    println!("Number of constraints: {}", solver.num_constraints());

    let status = solver.solve();
    println!("Status: {status}");

    if status.has_solution() {
        println!("Objective value: {}", solver.objective().value());
        println!("x = {}", x.solution_value());
        println!("y = {}", y.solution_value());
        println!("Wall time: {} ms", solver.wall_time_ms());
        println!("Iterations: {}", solver.iterations());
        println!("Nodes: {}", solver.nodes());
    }
}
