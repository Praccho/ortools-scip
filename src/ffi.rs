use std::ffi::{c_char, c_double, c_int, c_longlong, c_void};

extern "C" {
    // String deallocation
    pub fn or_free_string(s: *mut c_char);

    // Solver lifecycle
    pub fn or_solver_new(name: *const c_char) -> *mut c_void;
    pub fn or_solver_delete(s: *mut c_void);
    pub fn or_solver_name(s: *mut c_void, buf: *mut c_char, buf_len: c_int) -> c_int;
    pub fn or_solver_is_mip(s: *mut c_void) -> c_int;
    pub fn or_solver_clear(s: *mut c_void);
    pub fn or_solver_reset(s: *mut c_void);

    // Variables — creation & lookup
    pub fn or_solver_num_variables(s: *mut c_void) -> c_int;
    pub fn or_solver_variable(s: *mut c_void, index: c_int) -> *mut c_void;
    pub fn or_solver_lookup_variable(s: *mut c_void, name: *const c_char) -> *mut c_void;
    pub fn or_solver_make_var(
        s: *mut c_void,
        lb: c_double,
        ub: c_double,
        integer: c_int,
        name: *const c_char,
    ) -> *mut c_void;
    pub fn or_solver_make_num_var(
        s: *mut c_void,
        lb: c_double,
        ub: c_double,
        name: *const c_char,
    ) -> *mut c_void;
    pub fn or_solver_make_int_var(
        s: *mut c_void,
        lb: c_double,
        ub: c_double,
        name: *const c_char,
    ) -> *mut c_void;
    pub fn or_solver_make_bool_var(s: *mut c_void, name: *const c_char) -> *mut c_void;

    // Batch variable creation
    pub fn or_solver_make_var_array(
        s: *mut c_void,
        count: c_int,
        lb: c_double,
        ub: c_double,
        integer: c_int,
        prefix: *const c_char,
        out: *mut *mut c_void,
    );
    pub fn or_solver_make_num_var_array(
        s: *mut c_void,
        count: c_int,
        lb: c_double,
        ub: c_double,
        prefix: *const c_char,
        out: *mut *mut c_void,
    );
    pub fn or_solver_make_int_var_array(
        s: *mut c_void,
        count: c_int,
        lb: c_double,
        ub: c_double,
        prefix: *const c_char,
        out: *mut *mut c_void,
    );
    pub fn or_solver_make_bool_var_array(
        s: *mut c_void,
        count: c_int,
        prefix: *const c_char,
        out: *mut *mut c_void,
    );

    // Variable methods
    pub fn or_var_name(v: *mut c_void, buf: *mut c_char, buf_len: c_int) -> c_int;
    pub fn or_var_set_integer(v: *mut c_void, integer: c_int);
    pub fn or_var_integer(v: *mut c_void) -> c_int;
    pub fn or_var_solution_value(v: *mut c_void) -> c_double;
    pub fn or_var_index(v: *mut c_void) -> c_int;
    pub fn or_var_lb(v: *mut c_void) -> c_double;
    pub fn or_var_ub(v: *mut c_void) -> c_double;
    pub fn or_var_set_lb(v: *mut c_void, lb: c_double);
    pub fn or_var_set_ub(v: *mut c_void, ub: c_double);
    pub fn or_var_set_bounds(v: *mut c_void, lb: c_double, ub: c_double);
    pub fn or_var_unrounded_solution_value(v: *mut c_void) -> c_double;
    pub fn or_var_reduced_cost(v: *mut c_void) -> c_double;
    pub fn or_var_branching_priority(v: *mut c_void) -> c_int;
    pub fn or_var_set_branching_priority(v: *mut c_void, priority: c_int);

    // Constraints — creation & lookup
    pub fn or_solver_num_constraints(s: *mut c_void) -> c_int;
    pub fn or_solver_constraint(s: *mut c_void, index: c_int) -> *mut c_void;
    pub fn or_solver_lookup_constraint(s: *mut c_void, name: *const c_char) -> *mut c_void;
    pub fn or_solver_make_constraint(s: *mut c_void, lb: c_double, ub: c_double) -> *mut c_void;
    pub fn or_solver_make_constraint_named(
        s: *mut c_void,
        lb: c_double,
        ub: c_double,
        name: *const c_char,
    ) -> *mut c_void;
    pub fn or_solver_make_constraint_unbound(s: *mut c_void) -> *mut c_void;
    pub fn or_solver_make_constraint_unbound_named(
        s: *mut c_void,
        name: *const c_char,
    ) -> *mut c_void;

    // Constraint methods
    pub fn or_constraint_name(ct: *mut c_void, buf: *mut c_char, buf_len: c_int) -> c_int;
    pub fn or_constraint_clear(ct: *mut c_void);
    pub fn or_constraint_set_coefficient(ct: *mut c_void, var: *mut c_void, coeff: c_double);
    pub fn or_constraint_get_coefficient(ct: *mut c_void, var: *mut c_void) -> c_double;
    pub fn or_constraint_lb(ct: *mut c_void) -> c_double;
    pub fn or_constraint_ub(ct: *mut c_void) -> c_double;
    pub fn or_constraint_set_lb(ct: *mut c_void, lb: c_double);
    pub fn or_constraint_set_ub(ct: *mut c_void, ub: c_double);
    pub fn or_constraint_set_bounds(ct: *mut c_void, lb: c_double, ub: c_double);
    pub fn or_constraint_is_lazy(ct: *mut c_void) -> c_int;
    pub fn or_constraint_set_is_lazy(ct: *mut c_void, lazy: c_int);
    pub fn or_constraint_index(ct: *mut c_void) -> c_int;
    pub fn or_constraint_dual_value(ct: *mut c_void) -> c_double;

    // Objective
    pub fn or_objective_clear(s: *mut c_void);
    pub fn or_objective_set_coefficient(s: *mut c_void, var: *mut c_void, coeff: c_double);
    pub fn or_objective_get_coefficient(s: *mut c_void, var: *mut c_void) -> c_double;
    pub fn or_objective_set_offset(s: *mut c_void, offset: c_double);
    pub fn or_objective_offset(s: *mut c_void) -> c_double;
    pub fn or_objective_set_maximization(s: *mut c_void);
    pub fn or_objective_set_minimization(s: *mut c_void);
    pub fn or_objective_set_direction(s: *mut c_void, maximize: c_int);
    pub fn or_objective_maximization(s: *mut c_void) -> c_int;
    pub fn or_objective_minimization(s: *mut c_void) -> c_int;
    pub fn or_objective_value(s: *mut c_void) -> c_double;
    pub fn or_objective_best_bound(s: *mut c_void) -> c_double;

    // Solving
    pub fn or_solver_solve(s: *mut c_void) -> c_int;
    pub fn or_solver_solve_with_params(s: *mut c_void, p: *mut c_void) -> c_int;
    pub fn or_solver_next_solution(s: *mut c_void) -> c_int;
    pub fn or_solver_interrupt(s: *mut c_void) -> c_int;

    // Solver settings
    pub fn or_solver_set_time_limit_ms(s: *mut c_void, ms: c_longlong);
    pub fn or_solver_time_limit_ms(s: *mut c_void) -> c_longlong;
    pub fn or_solver_set_num_threads(s: *mut c_void, num: c_int) -> c_int;
    pub fn or_solver_get_num_threads(s: *mut c_void) -> c_int;
    pub fn or_solver_set_specific_params(s: *mut c_void, params: *const c_char) -> c_int;
    pub fn or_solver_enable_output(s: *mut c_void);
    pub fn or_solver_suppress_output(s: *mut c_void);

    // Hints
    pub fn or_solver_set_hint(
        s: *mut c_void,
        vars: *mut *mut c_void,
        values: *const c_double,
        count: c_int,
    );

    // Info & export
    pub fn or_solver_version(s: *mut c_void, buf: *mut c_char, buf_len: c_int) -> c_int;
    pub fn or_solver_infinity() -> c_double;
    pub fn or_solver_iterations(s: *mut c_void) -> c_longlong;
    pub fn or_solver_nodes(s: *mut c_void) -> c_longlong;
    pub fn or_solver_wall_time(s: *mut c_void) -> c_longlong;
    pub fn or_solver_compute_exact_condition_number(s: *mut c_void) -> c_double;
    pub fn or_solver_verify_solution(s: *mut c_void, tolerance: c_double, log: c_int) -> c_int;
    pub fn or_solver_write(s: *mut c_void, filename: *const c_char);
    pub fn or_solver_export_lp(s: *mut c_void, obfuscate: c_int) -> *mut c_char;
    pub fn or_solver_export_mps(
        s: *mut c_void,
        fixed_format: c_int,
        obfuscate: c_int,
    ) -> *mut c_char;

    // Constraint activities
    pub fn or_solver_compute_constraint_activities(
        s: *mut c_void,
        out: *mut c_double,
        count: c_int,
    );

    // Solver parameters
    pub fn or_params_new() -> *mut c_void;
    pub fn or_params_delete(p: *mut c_void);
    pub fn or_params_set_double(p: *mut c_void, param: c_int, value: c_double);
    pub fn or_params_get_double(p: *mut c_void, param: c_int) -> c_double;
    pub fn or_params_set_integer(p: *mut c_void, param: c_int, value: c_int);
    pub fn or_params_get_integer(p: *mut c_void, param: c_int) -> c_int;
    pub fn or_params_reset(p: *mut c_void);
    pub fn or_params_reset_double(p: *mut c_void, param: c_int);
    pub fn or_params_reset_integer(p: *mut c_void, param: c_int);
}
