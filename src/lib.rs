//! Rust bindings for the Google OR-Tools SCIP mixed-integer programming solver.
//!
//! # Quick start
//!
//! ```no_run
//! use ortools_scip::Solver;
//!
//! let mut solver = Solver::new("example");
//!
//! // Continuous variables: x >= 0, y >= 0
//! let x = solver.make_num_var(0.0, f64::INFINITY, "x");
//! let y = solver.make_num_var(0.0, f64::INFINITY, "y");
//!
//! // x + 2y <= 14
//! let ct = solver.make_constraint(f64::NEG_INFINITY, 14.0);
//! ct.set_coefficient(&x, 1.0);
//! ct.set_coefficient(&y, 2.0);
//!
//! // 3x + y <= 14
//! let ct2 = solver.make_constraint(f64::NEG_INFINITY, 14.0);
//! ct2.set_coefficient(&x, 3.0);
//! ct2.set_coefficient(&y, 1.0);
//!
//! // maximize 3x + 5y
//! solver.objective().set_coefficient(&x, 3.0);
//! solver.objective().set_coefficient(&y, 5.0);
//! solver.objective().set_maximization();
//!
//! let status = solver.solve();
//! if status.has_solution() {
//!     println!("x = {}, y = {}", x.solution_value(), y.solution_value());
//!     println!("objective = {}", solver.objective().value());
//! }
//! ```

mod ffi;

use std::ffi::{CStr, CString};
use std::ptr;

// ---------------------------------------------------------------------------
// Status
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Status {
    Optimal = 0,
    Feasible = 1,
    Infeasible = 2,
    Unbounded = 3,
    Abnormal = 4,
    ModelInvalid = 5,
    NotSolved = 6,
}

impl Status {
    fn from_raw(v: i32) -> Self {
        match v {
            0 => Self::Optimal,
            1 => Self::Feasible,
            2 => Self::Infeasible,
            3 => Self::Unbounded,
            4 => Self::Abnormal,
            5 => Self::ModelInvalid,
            _ => Self::NotSolved,
        }
    }

    pub fn is_optimal(self) -> bool {
        self == Self::Optimal
    }

    pub fn has_solution(self) -> bool {
        matches!(self, Self::Optimal | Self::Feasible)
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Optimal => write!(f, "OPTIMAL"),
            Self::Feasible => write!(f, "FEASIBLE"),
            Self::Infeasible => write!(f, "INFEASIBLE"),
            Self::Unbounded => write!(f, "UNBOUNDED"),
            Self::Abnormal => write!(f, "ABNORMAL"),
            Self::ModelInvalid => write!(f, "MODEL_INVALID"),
            Self::NotSolved => write!(f, "NOT_SOLVED"),
        }
    }
}

// ---------------------------------------------------------------------------
// Solver parameter enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DoubleParam {
    RelativeMipGap = 0,
    PrimalTolerance = 1,
    DualTolerance = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum IntegerParam {
    Presolve = 1000,
    LpAlgorithm = 1001,
    Incrementality = 1002,
    Scaling = 1003,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PresolveValues {
    Off = 0,
    On = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LpAlgorithmValues {
    Dual = 10,
    Primal = 11,
    Barrier = 12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum IncrementalityValues {
    Off = 0,
    On = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ScalingValues {
    Off = 0,
    On = 1,
}

// ---------------------------------------------------------------------------
// SolverParams
// ---------------------------------------------------------------------------

pub struct SolverParams {
    ptr: *mut std::ffi::c_void,
}

impl SolverParams {
    pub fn new() -> Self {
        Self {
            ptr: unsafe { ffi::or_params_new() },
        }
    }

    pub fn set_double(&mut self, param: DoubleParam, value: f64) {
        unsafe { ffi::or_params_set_double(self.ptr, param as i32, value) }
    }

    pub fn get_double(&self, param: DoubleParam) -> f64 {
        unsafe { ffi::or_params_get_double(self.ptr, param as i32) }
    }

    pub fn set_integer(&mut self, param: IntegerParam, value: i32) {
        unsafe { ffi::or_params_set_integer(self.ptr, param as i32, value) }
    }

    pub fn get_integer(&self, param: IntegerParam) -> i32 {
        unsafe { ffi::or_params_get_integer(self.ptr, param as i32) }
    }

    pub fn reset(&mut self) {
        unsafe { ffi::or_params_reset(self.ptr) }
    }

    pub fn reset_double(&mut self, param: DoubleParam) {
        unsafe { ffi::or_params_reset_double(self.ptr, param as i32) }
    }

    pub fn reset_integer(&mut self, param: IntegerParam) {
        unsafe { ffi::or_params_reset_integer(self.ptr, param as i32) }
    }
}

impl Default for SolverParams {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SolverParams {
    fn drop(&mut self) {
        unsafe { ffi::or_params_delete(self.ptr) }
    }
}

// ---------------------------------------------------------------------------
// Variable
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct Variable {
    ptr: *mut std::ffi::c_void,
}

impl Variable {
    pub fn name(&self) -> String {
        read_string(|buf, len| unsafe { ffi::or_var_name(self.ptr, buf, len) })
    }

    pub fn set_integer(&self, integer: bool) {
        unsafe { ffi::or_var_set_integer(self.ptr, integer as i32) }
    }

    pub fn integer(&self) -> bool {
        unsafe { ffi::or_var_integer(self.ptr) != 0 }
    }

    pub fn solution_value(&self) -> f64 {
        unsafe { ffi::or_var_solution_value(self.ptr) }
    }

    pub fn index(&self) -> i32 {
        unsafe { ffi::or_var_index(self.ptr) }
    }

    pub fn lb(&self) -> f64 {
        unsafe { ffi::or_var_lb(self.ptr) }
    }

    pub fn ub(&self) -> f64 {
        unsafe { ffi::or_var_ub(self.ptr) }
    }

    pub fn set_lb(&self, lb: f64) {
        unsafe { ffi::or_var_set_lb(self.ptr, lb) }
    }

    pub fn set_ub(&self, ub: f64) {
        unsafe { ffi::or_var_set_ub(self.ptr, ub) }
    }

    pub fn set_bounds(&self, lb: f64, ub: f64) {
        unsafe { ffi::or_var_set_bounds(self.ptr, lb, ub) }
    }

    pub fn unrounded_solution_value(&self) -> f64 {
        unsafe { ffi::or_var_unrounded_solution_value(self.ptr) }
    }

    pub fn reduced_cost(&self) -> f64 {
        unsafe { ffi::or_var_reduced_cost(self.ptr) }
    }

    pub fn branching_priority(&self) -> i32 {
        unsafe { ffi::or_var_branching_priority(self.ptr) }
    }

    pub fn set_branching_priority(&self, priority: i32) {
        unsafe { ffi::or_var_set_branching_priority(self.ptr, priority) }
    }
}

impl std::fmt::Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Variable")
            .field("name", &self.name())
            .field("lb", &self.lb())
            .field("ub", &self.ub())
            .field("integer", &self.integer())
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Constraint
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct Constraint {
    ptr: *mut std::ffi::c_void,
}

impl Constraint {
    pub fn name(&self) -> String {
        read_string(|buf, len| unsafe { ffi::or_constraint_name(self.ptr, buf, len) })
    }

    pub fn clear(&self) {
        unsafe { ffi::or_constraint_clear(self.ptr) }
    }

    pub fn set_coefficient(&self, var: &Variable, coeff: f64) {
        unsafe { ffi::or_constraint_set_coefficient(self.ptr, var.ptr, coeff) }
    }

    pub fn get_coefficient(&self, var: &Variable) -> f64 {
        unsafe { ffi::or_constraint_get_coefficient(self.ptr, var.ptr) }
    }

    pub fn lb(&self) -> f64 {
        unsafe { ffi::or_constraint_lb(self.ptr) }
    }

    pub fn ub(&self) -> f64 {
        unsafe { ffi::or_constraint_ub(self.ptr) }
    }

    pub fn set_lb(&self, lb: f64) {
        unsafe { ffi::or_constraint_set_lb(self.ptr, lb) }
    }

    pub fn set_ub(&self, ub: f64) {
        unsafe { ffi::or_constraint_set_ub(self.ptr, ub) }
    }

    pub fn set_bounds(&self, lb: f64, ub: f64) {
        unsafe { ffi::or_constraint_set_bounds(self.ptr, lb, ub) }
    }

    pub fn is_lazy(&self) -> bool {
        unsafe { ffi::or_constraint_is_lazy(self.ptr) != 0 }
    }

    pub fn set_lazy(&self, lazy: bool) {
        unsafe { ffi::or_constraint_set_is_lazy(self.ptr, lazy as i32) }
    }

    pub fn index(&self) -> i32 {
        unsafe { ffi::or_constraint_index(self.ptr) }
    }

    pub fn dual_value(&self) -> f64 {
        unsafe { ffi::or_constraint_dual_value(self.ptr) }
    }
}

impl std::fmt::Debug for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Constraint")
            .field("name", &self.name())
            .field("lb", &self.lb())
            .field("ub", &self.ub())
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Objective
// ---------------------------------------------------------------------------

pub struct Objective<'a> {
    solver: &'a Solver,
}

impl Objective<'_> {
    pub fn clear(&self) {
        unsafe { ffi::or_objective_clear(self.solver.ptr) }
    }

    pub fn set_coefficient(&self, var: &Variable, coeff: f64) {
        unsafe { ffi::or_objective_set_coefficient(self.solver.ptr, var.ptr, coeff) }
    }

    pub fn get_coefficient(&self, var: &Variable) -> f64 {
        unsafe { ffi::or_objective_get_coefficient(self.solver.ptr, var.ptr) }
    }

    pub fn set_offset(&self, offset: f64) {
        unsafe { ffi::or_objective_set_offset(self.solver.ptr, offset) }
    }

    pub fn offset(&self) -> f64 {
        unsafe { ffi::or_objective_offset(self.solver.ptr) }
    }

    pub fn set_maximization(&self) {
        unsafe { ffi::or_objective_set_maximization(self.solver.ptr) }
    }

    pub fn set_minimization(&self) {
        unsafe { ffi::or_objective_set_minimization(self.solver.ptr) }
    }

    pub fn set_direction(&self, maximize: bool) {
        unsafe { ffi::or_objective_set_direction(self.solver.ptr, maximize as i32) }
    }

    pub fn is_maximization(&self) -> bool {
        unsafe { ffi::or_objective_maximization(self.solver.ptr) != 0 }
    }

    pub fn is_minimization(&self) -> bool {
        unsafe { ffi::or_objective_minimization(self.solver.ptr) != 0 }
    }

    pub fn value(&self) -> f64 {
        unsafe { ffi::or_objective_value(self.solver.ptr) }
    }

    pub fn best_bound(&self) -> f64 {
        unsafe { ffi::or_objective_best_bound(self.solver.ptr) }
    }
}

// ---------------------------------------------------------------------------
// Solver
// ---------------------------------------------------------------------------

pub struct Solver {
    ptr: *mut std::ffi::c_void,
}

impl Solver {
    pub fn new(name: &str) -> Self {
        let c_name = CString::new(name).unwrap();
        Self {
            ptr: unsafe { ffi::or_solver_new(c_name.as_ptr()) },
        }
    }

    pub fn name(&self) -> String {
        read_string(|buf, len| unsafe { ffi::or_solver_name(self.ptr, buf, len) })
    }

    pub fn is_mip(&self) -> bool {
        unsafe { ffi::or_solver_is_mip(self.ptr) != 0 }
    }

    pub fn clear(&mut self) {
        unsafe { ffi::or_solver_clear(self.ptr) }
    }

    pub fn reset(&mut self) {
        unsafe { ffi::or_solver_reset(self.ptr) }
    }

    // -- Variables -----------------------------------------------------------

    pub fn num_variables(&self) -> i32 {
        unsafe { ffi::or_solver_num_variables(self.ptr) }
    }

    pub fn variable(&self, index: i32) -> Variable {
        Variable {
            ptr: unsafe { ffi::or_solver_variable(self.ptr, index) },
        }
    }

    pub fn lookup_variable(&self, name: &str) -> Option<Variable> {
        let c_name = CString::new(name).unwrap();
        let ptr = unsafe { ffi::or_solver_lookup_variable(self.ptr, c_name.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Variable { ptr })
        }
    }

    pub fn make_var(&mut self, lb: f64, ub: f64, integer: bool, name: &str) -> Variable {
        let c_name = CString::new(name).unwrap();
        Variable {
            ptr: unsafe {
                ffi::or_solver_make_var(self.ptr, lb, ub, integer as i32, c_name.as_ptr())
            },
        }
    }

    pub fn make_num_var(&mut self, lb: f64, ub: f64, name: &str) -> Variable {
        let c_name = CString::new(name).unwrap();
        Variable {
            ptr: unsafe { ffi::or_solver_make_num_var(self.ptr, lb, ub, c_name.as_ptr()) },
        }
    }

    pub fn make_int_var(&mut self, lb: f64, ub: f64, name: &str) -> Variable {
        let c_name = CString::new(name).unwrap();
        Variable {
            ptr: unsafe { ffi::or_solver_make_int_var(self.ptr, lb, ub, c_name.as_ptr()) },
        }
    }

    pub fn make_bool_var(&mut self, name: &str) -> Variable {
        let c_name = CString::new(name).unwrap();
        Variable {
            ptr: unsafe { ffi::or_solver_make_bool_var(self.ptr, c_name.as_ptr()) },
        }
    }

    pub fn make_var_array(
        &mut self,
        count: usize,
        lb: f64,
        ub: f64,
        integer: bool,
        prefix: &str,
    ) -> Vec<Variable> {
        let c_prefix = CString::new(prefix).unwrap();
        let mut ptrs = vec![ptr::null_mut(); count];
        unsafe {
            ffi::or_solver_make_var_array(
                self.ptr,
                count as i32,
                lb,
                ub,
                integer as i32,
                c_prefix.as_ptr(),
                ptrs.as_mut_ptr(),
            );
        }
        ptrs.into_iter().map(|ptr| Variable { ptr }).collect()
    }

    pub fn make_num_var_array(
        &mut self,
        count: usize,
        lb: f64,
        ub: f64,
        prefix: &str,
    ) -> Vec<Variable> {
        let c_prefix = CString::new(prefix).unwrap();
        let mut ptrs = vec![ptr::null_mut(); count];
        unsafe {
            ffi::or_solver_make_num_var_array(
                self.ptr,
                count as i32,
                lb,
                ub,
                c_prefix.as_ptr(),
                ptrs.as_mut_ptr(),
            );
        }
        ptrs.into_iter().map(|ptr| Variable { ptr }).collect()
    }

    pub fn make_int_var_array(
        &mut self,
        count: usize,
        lb: f64,
        ub: f64,
        prefix: &str,
    ) -> Vec<Variable> {
        let c_prefix = CString::new(prefix).unwrap();
        let mut ptrs = vec![ptr::null_mut(); count];
        unsafe {
            ffi::or_solver_make_int_var_array(
                self.ptr,
                count as i32,
                lb,
                ub,
                c_prefix.as_ptr(),
                ptrs.as_mut_ptr(),
            );
        }
        ptrs.into_iter().map(|ptr| Variable { ptr }).collect()
    }

    pub fn make_bool_var_array(&mut self, count: usize, prefix: &str) -> Vec<Variable> {
        let c_prefix = CString::new(prefix).unwrap();
        let mut ptrs = vec![ptr::null_mut(); count];
        unsafe {
            ffi::or_solver_make_bool_var_array(
                self.ptr,
                count as i32,
                c_prefix.as_ptr(),
                ptrs.as_mut_ptr(),
            );
        }
        ptrs.into_iter().map(|ptr| Variable { ptr }).collect()
    }

    // -- Constraints ---------------------------------------------------------

    pub fn num_constraints(&self) -> i32 {
        unsafe { ffi::or_solver_num_constraints(self.ptr) }
    }

    pub fn constraint(&self, index: i32) -> Constraint {
        Constraint {
            ptr: unsafe { ffi::or_solver_constraint(self.ptr, index) },
        }
    }

    pub fn lookup_constraint(&self, name: &str) -> Option<Constraint> {
        let c_name = CString::new(name).unwrap();
        let ptr = unsafe { ffi::or_solver_lookup_constraint(self.ptr, c_name.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Constraint { ptr })
        }
    }

    pub fn make_constraint(&mut self, lb: f64, ub: f64) -> Constraint {
        Constraint {
            ptr: unsafe { ffi::or_solver_make_constraint(self.ptr, lb, ub) },
        }
    }

    pub fn make_constraint_named(&mut self, lb: f64, ub: f64, name: &str) -> Constraint {
        let c_name = CString::new(name).unwrap();
        Constraint {
            ptr: unsafe {
                ffi::or_solver_make_constraint_named(self.ptr, lb, ub, c_name.as_ptr())
            },
        }
    }

    pub fn make_constraint_unbound(&mut self) -> Constraint {
        Constraint {
            ptr: unsafe { ffi::or_solver_make_constraint_unbound(self.ptr) },
        }
    }

    pub fn make_constraint_unbound_named(&mut self, name: &str) -> Constraint {
        let c_name = CString::new(name).unwrap();
        Constraint {
            ptr: unsafe {
                ffi::or_solver_make_constraint_unbound_named(self.ptr, c_name.as_ptr())
            },
        }
    }

    // -- Objective -----------------------------------------------------------

    pub fn objective(&self) -> Objective<'_> {
        Objective { solver: self }
    }

    // -- Solving -------------------------------------------------------------

    pub fn solve(&mut self) -> Status {
        Status::from_raw(unsafe { ffi::or_solver_solve(self.ptr) })
    }

    pub fn solve_with_params(&mut self, params: &SolverParams) -> Status {
        Status::from_raw(unsafe { ffi::or_solver_solve_with_params(self.ptr, params.ptr) })
    }

    pub fn next_solution(&mut self) -> bool {
        unsafe { ffi::or_solver_next_solution(self.ptr) != 0 }
    }

    pub fn interrupt(&self) -> bool {
        unsafe { ffi::or_solver_interrupt(self.ptr) != 0 }
    }

    // -- Settings ------------------------------------------------------------

    pub fn set_time_limit_ms(&mut self, ms: i64) {
        unsafe { ffi::or_solver_set_time_limit_ms(self.ptr, ms) }
    }

    pub fn time_limit_ms(&self) -> i64 {
        unsafe { ffi::or_solver_time_limit_ms(self.ptr) }
    }

    pub fn set_num_threads(&mut self, num: i32) -> Result<(), ()> {
        let ret = unsafe { ffi::or_solver_set_num_threads(self.ptr, num) };
        if ret == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn num_threads(&self) -> i32 {
        unsafe { ffi::or_solver_get_num_threads(self.ptr) }
    }

    pub fn set_solver_specific_parameters(&mut self, params: &str) -> bool {
        let c_params = CString::new(params).unwrap();
        unsafe { ffi::or_solver_set_specific_params(self.ptr, c_params.as_ptr()) != 0 }
    }

    pub fn enable_output(&mut self) {
        unsafe { ffi::or_solver_enable_output(self.ptr) }
    }

    pub fn suppress_output(&mut self) {
        unsafe { ffi::or_solver_suppress_output(self.ptr) }
    }

    // -- Hints ---------------------------------------------------------------

    pub fn set_hint(&mut self, hints: &[(&Variable, f64)]) {
        let mut var_ptrs: Vec<*mut std::ffi::c_void> =
            hints.iter().map(|(v, _)| v.ptr).collect();
        let values: Vec<f64> = hints.iter().map(|(_, val)| *val).collect();
        unsafe {
            ffi::or_solver_set_hint(
                self.ptr,
                var_ptrs.as_mut_ptr(),
                values.as_ptr(),
                hints.len() as i32,
            );
        }
    }

    // -- Info ----------------------------------------------------------------

    pub fn version(&self) -> String {
        read_string(|buf, len| unsafe { ffi::or_solver_version(self.ptr, buf, len) })
    }

    pub fn infinity() -> f64 {
        unsafe { ffi::or_solver_infinity() }
    }

    pub fn iterations(&self) -> i64 {
        unsafe { ffi::or_solver_iterations(self.ptr) }
    }

    pub fn nodes(&self) -> i64 {
        unsafe { ffi::or_solver_nodes(self.ptr) }
    }

    pub fn wall_time_ms(&self) -> i64 {
        unsafe { ffi::or_solver_wall_time(self.ptr) }
    }

    pub fn compute_exact_condition_number(&self) -> f64 {
        unsafe { ffi::or_solver_compute_exact_condition_number(self.ptr) }
    }

    pub fn verify_solution(&self, tolerance: f64, log_errors: bool) -> bool {
        unsafe { ffi::or_solver_verify_solution(self.ptr, tolerance, log_errors as i32) != 0 }
    }

    // -- Export --------------------------------------------------------------

    pub fn write(&self, filename: &str) {
        let c_name = CString::new(filename).unwrap();
        unsafe { ffi::or_solver_write(self.ptr, c_name.as_ptr()) }
    }

    pub fn export_as_lp(&self, obfuscate: bool) -> Option<String> {
        let ptr = unsafe { ffi::or_solver_export_lp(self.ptr, obfuscate as i32) };
        if ptr.is_null() {
            return None;
        }
        let s = unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned();
        unsafe { ffi::or_free_string(ptr) };
        Some(s)
    }

    pub fn export_as_mps(&self, fixed_format: bool, obfuscate: bool) -> Option<String> {
        let ptr = unsafe {
            ffi::or_solver_export_mps(self.ptr, fixed_format as i32, obfuscate as i32)
        };
        if ptr.is_null() {
            return None;
        }
        let s = unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned();
        unsafe { ffi::or_free_string(ptr) };
        Some(s)
    }

    pub fn compute_constraint_activities(&self) -> Vec<f64> {
        let n = self.num_constraints() as usize;
        let mut activities = vec![0.0; n];
        unsafe {
            ffi::or_solver_compute_constraint_activities(
                self.ptr,
                activities.as_mut_ptr(),
                n as i32,
            );
        }
        activities
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        unsafe { ffi::or_solver_delete(self.ptr) }
    }
}

unsafe impl Send for Solver {}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn read_string(f: impl Fn(*mut i8, i32) -> i32) -> String {
    let needed = f(ptr::null_mut(), 0);
    if needed <= 0 {
        return String::new();
    }
    let buf_len = needed + 1;
    let mut buf = vec![0i8; buf_len as usize];
    f(buf.as_mut_ptr(), buf_len);
    unsafe { CStr::from_ptr(buf.as_ptr()) }
        .to_string_lossy()
        .into_owned()
}
