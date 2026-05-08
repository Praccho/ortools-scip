#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>

#include "ortools/linear_solver/linear_solver.h"

namespace ort = operations_research;

using Solver = ort::MPSolver;
using Variable = ort::MPVariable;
using Constraint = ort::MPConstraint;
using Objective = ort::MPObjective;
using Params = ort::MPSolverParameters;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

static int copy_string(const std::string& src, char* buf, int buf_len) {
    if (buf && buf_len > 0) {
        int n = static_cast<int>(src.size());
        if (n >= buf_len) n = buf_len - 1;
        std::memcpy(buf, src.data(), n);
        buf[n] = '\0';
        return n;
    }
    return static_cast<int>(src.size());
}

static char* alloc_string(const std::string& src) {
    char* s = static_cast<char*>(std::malloc(src.size() + 1));
    if (s) {
        std::memcpy(s, src.data(), src.size());
        s[src.size()] = '\0';
    }
    return s;
}

extern "C" {

// ---------------------------------------------------------------------------
// String deallocation
// ---------------------------------------------------------------------------

void or_free_string(char* s) { std::free(s); }

// ---------------------------------------------------------------------------
// Solver lifecycle
// ---------------------------------------------------------------------------

Solver* or_solver_new(const char* name) {
    return new Solver(name ? name : "",
                      Solver::SCIP_MIXED_INTEGER_PROGRAMMING);
}

void or_solver_delete(Solver* s) { delete s; }

int or_solver_name(Solver* s, char* buf, int buf_len) {
    return copy_string(s->Name(), buf, buf_len);
}

int or_solver_is_mip(Solver* s) { return s->IsMIP() ? 1 : 0; }

void or_solver_clear(Solver* s) { s->Clear(); }

void or_solver_reset(Solver* s) { s->Reset(); }

// ---------------------------------------------------------------------------
// Variables — creation & lookup
// ---------------------------------------------------------------------------

int or_solver_num_variables(Solver* s) { return s->NumVariables(); }

Variable* or_solver_variable(Solver* s, int index) {
    return s->variable(index);
}

Variable* or_solver_lookup_variable(Solver* s, const char* name) {
    return s->LookupVariableOrNull(name ? name : "");
}

Variable* or_solver_make_var(Solver* s, double lb, double ub, int integer,
                             const char* name) {
    return s->MakeVar(lb, ub, integer != 0, name ? name : "");
}

Variable* or_solver_make_num_var(Solver* s, double lb, double ub,
                                 const char* name) {
    return s->MakeNumVar(lb, ub, name ? name : "");
}

Variable* or_solver_make_int_var(Solver* s, double lb, double ub,
                                 const char* name) {
    return s->MakeIntVar(lb, ub, name ? name : "");
}

Variable* or_solver_make_bool_var(Solver* s, const char* name) {
    return s->MakeBoolVar(name ? name : "");
}

// Batch variable creation
void or_solver_make_var_array(Solver* s, int count, double lb, double ub,
                              int integer, const char* prefix,
                              Variable** out) {
    std::vector<Variable*> vars;
    s->MakeVarArray(count, lb, ub, integer != 0, prefix ? prefix : "", &vars);
    for (int i = 0; i < count && i < static_cast<int>(vars.size()); i++) {
        out[i] = vars[i];
    }
}

void or_solver_make_num_var_array(Solver* s, int count, double lb, double ub,
                                  const char* prefix, Variable** out) {
    std::vector<Variable*> vars;
    s->MakeNumVarArray(count, lb, ub, prefix ? prefix : "", &vars);
    for (int i = 0; i < count && i < static_cast<int>(vars.size()); i++) {
        out[i] = vars[i];
    }
}

void or_solver_make_int_var_array(Solver* s, int count, double lb, double ub,
                                  const char* prefix, Variable** out) {
    std::vector<Variable*> vars;
    s->MakeIntVarArray(count, lb, ub, prefix ? prefix : "", &vars);
    for (int i = 0; i < count && i < static_cast<int>(vars.size()); i++) {
        out[i] = vars[i];
    }
}

void or_solver_make_bool_var_array(Solver* s, int count, const char* prefix,
                                   Variable** out) {
    std::vector<Variable*> vars;
    s->MakeBoolVarArray(count, prefix ? prefix : "", &vars);
    for (int i = 0; i < count && i < static_cast<int>(vars.size()); i++) {
        out[i] = vars[i];
    }
}

// ---------------------------------------------------------------------------
// Variable methods
// ---------------------------------------------------------------------------

int or_var_name(Variable* v, char* buf, int buf_len) {
    return copy_string(v->name(), buf, buf_len);
}

void or_var_set_integer(Variable* v, int integer) {
    v->SetInteger(integer != 0);
}

int or_var_integer(Variable* v) { return v->integer() ? 1 : 0; }

double or_var_solution_value(Variable* v) { return v->solution_value(); }

int or_var_index(Variable* v) { return v->index(); }

double or_var_lb(Variable* v) { return v->lb(); }

double or_var_ub(Variable* v) { return v->ub(); }

void or_var_set_lb(Variable* v, double lb) { v->SetLB(lb); }

void or_var_set_ub(Variable* v, double ub) { v->SetUB(ub); }

void or_var_set_bounds(Variable* v, double lb, double ub) {
    v->SetBounds(lb, ub);
}

double or_var_unrounded_solution_value(Variable* v) {
    return v->unrounded_solution_value();
}

double or_var_reduced_cost(Variable* v) { return v->reduced_cost(); }

int or_var_branching_priority(Variable* v) {
    return v->branching_priority();
}

void or_var_set_branching_priority(Variable* v, int priority) {
    v->SetBranchingPriority(priority);
}

// ---------------------------------------------------------------------------
// Constraints — creation & lookup
// ---------------------------------------------------------------------------

int or_solver_num_constraints(Solver* s) { return s->NumConstraints(); }

Constraint* or_solver_constraint(Solver* s, int index) {
    return s->constraint(index);
}

Constraint* or_solver_lookup_constraint(Solver* s, const char* name) {
    return s->LookupConstraintOrNull(name ? name : "");
}

Constraint* or_solver_make_constraint(Solver* s, double lb, double ub) {
    return s->MakeRowConstraint(lb, ub);
}

Constraint* or_solver_make_constraint_named(Solver* s, double lb, double ub,
                                            const char* name) {
    return s->MakeRowConstraint(lb, ub, name ? name : "");
}

Constraint* or_solver_make_constraint_unbound(Solver* s) {
    return s->MakeRowConstraint();
}

Constraint* or_solver_make_constraint_unbound_named(Solver* s,
                                                     const char* name) {
    return s->MakeRowConstraint(name ? name : "");
}

// ---------------------------------------------------------------------------
// Constraint methods
// ---------------------------------------------------------------------------

int or_constraint_name(Constraint* ct, char* buf, int buf_len) {
    return copy_string(ct->name(), buf, buf_len);
}

void or_constraint_clear(Constraint* ct) { ct->Clear(); }

void or_constraint_set_coefficient(Constraint* ct, Variable* var,
                                   double coeff) {
    ct->SetCoefficient(var, coeff);
}

double or_constraint_get_coefficient(Constraint* ct, Variable* var) {
    return ct->GetCoefficient(var);
}

double or_constraint_lb(Constraint* ct) { return ct->lb(); }

double or_constraint_ub(Constraint* ct) { return ct->ub(); }

void or_constraint_set_lb(Constraint* ct, double lb) { ct->SetLB(lb); }

void or_constraint_set_ub(Constraint* ct, double ub) { ct->SetUB(ub); }

void or_constraint_set_bounds(Constraint* ct, double lb, double ub) {
    ct->SetBounds(lb, ub);
}

int or_constraint_is_lazy(Constraint* ct) { return ct->is_lazy() ? 1 : 0; }

void or_constraint_set_is_lazy(Constraint* ct, int lazy) {
    ct->set_is_lazy(lazy != 0);
}

int or_constraint_index(Constraint* ct) { return ct->index(); }

double or_constraint_dual_value(Constraint* ct) { return ct->dual_value(); }

// ---------------------------------------------------------------------------
// Objective
// ---------------------------------------------------------------------------

void or_objective_clear(Solver* s) { s->MutableObjective()->Clear(); }

void or_objective_set_coefficient(Solver* s, Variable* var, double coeff) {
    s->MutableObjective()->SetCoefficient(var, coeff);
}

double or_objective_get_coefficient(Solver* s, Variable* var) {
    return s->MutableObjective()->GetCoefficient(var);
}

void or_objective_set_offset(Solver* s, double offset) {
    s->MutableObjective()->SetOffset(offset);
}

double or_objective_offset(Solver* s) {
    return s->MutableObjective()->offset();
}

void or_objective_set_maximization(Solver* s) {
    s->MutableObjective()->SetMaximization();
}

void or_objective_set_minimization(Solver* s) {
    s->MutableObjective()->SetMinimization();
}

void or_objective_set_direction(Solver* s, int maximize) {
    s->MutableObjective()->SetOptimizationDirection(maximize != 0);
}

int or_objective_maximization(Solver* s) {
    return s->MutableObjective()->maximization() ? 1 : 0;
}

int or_objective_minimization(Solver* s) {
    return s->MutableObjective()->minimization() ? 1 : 0;
}

double or_objective_value(Solver* s) {
    return s->MutableObjective()->Value();
}

double or_objective_best_bound(Solver* s) {
    return s->MutableObjective()->BestBound();
}

// ---------------------------------------------------------------------------
// Solving
// ---------------------------------------------------------------------------

int or_solver_solve(Solver* s) { return static_cast<int>(s->Solve()); }

int or_solver_solve_with_params(Solver* s, Params* p) {
    return static_cast<int>(s->Solve(*p));
}

int or_solver_next_solution(Solver* s) { return s->NextSolution() ? 1 : 0; }

int or_solver_interrupt(Solver* s) {
    return s->InterruptSolve() ? 1 : 0;
}

// ---------------------------------------------------------------------------
// Solver settings
// ---------------------------------------------------------------------------

void or_solver_set_time_limit_ms(Solver* s, int64_t ms) {
    s->set_time_limit(ms);
}

int64_t or_solver_time_limit_ms(Solver* s) { return s->time_limit(); }

int or_solver_set_num_threads(Solver* s, int num) {
    auto status = s->SetNumThreads(num);
    return status.ok() ? 0 : -1;
}

int or_solver_get_num_threads(Solver* s) { return s->GetNumThreads(); }

int or_solver_set_specific_params(Solver* s, const char* params) {
    return s->SetSolverSpecificParametersAsString(params ? params : "") ? 1
                                                                        : 0;
}

void or_solver_enable_output(Solver* s) { s->EnableOutput(); }

void or_solver_suppress_output(Solver* s) { s->SuppressOutput(); }

// ---------------------------------------------------------------------------
// Hints
// ---------------------------------------------------------------------------

void or_solver_set_hint(Solver* s, Variable** vars, const double* values,
                        int count) {
    std::vector<std::pair<const Variable*, double>> hint;
    hint.reserve(count);
    for (int i = 0; i < count; i++) {
        hint.emplace_back(vars[i], values[i]);
    }
    s->SetHint(std::move(hint));
}

// ---------------------------------------------------------------------------
// Info & export
// ---------------------------------------------------------------------------

int or_solver_version(Solver* s, char* buf, int buf_len) {
    return copy_string(s->SolverVersion(), buf, buf_len);
}

double or_solver_infinity() { return Solver::infinity(); }

int64_t or_solver_iterations(Solver* s) { return s->iterations(); }

int64_t or_solver_nodes(Solver* s) { return s->nodes(); }

int64_t or_solver_wall_time(Solver* s) { return s->wall_time(); }

double or_solver_compute_exact_condition_number(Solver* s) {
    return s->ComputeExactConditionNumber();
}

int or_solver_verify_solution(Solver* s, double tolerance, int log_errors) {
    return s->VerifySolution(tolerance, log_errors != 0) ? 1 : 0;
}

void or_solver_write(Solver* s, const char* filename) {
    s->Write(filename ? filename : "");
}

char* or_solver_export_lp(Solver* s, int obfuscate) {
    std::string model;
    if (s->ExportModelAsLpFormat(obfuscate != 0, &model)) {
        return alloc_string(model);
    }
    return nullptr;
}

char* or_solver_export_mps(Solver* s, int fixed_format, int obfuscate) {
    std::string model;
    if (s->ExportModelAsMpsFormat(fixed_format != 0, obfuscate != 0, &model)) {
        return alloc_string(model);
    }
    return nullptr;
}

// ---------------------------------------------------------------------------
// Constraint activities
// ---------------------------------------------------------------------------

void or_solver_compute_constraint_activities(Solver* s, double* out,
                                             int count) {
    auto acts = s->ComputeConstraintActivities();
    int n = static_cast<int>(acts.size());
    if (n > count) n = count;
    for (int i = 0; i < n; i++) {
        out[i] = acts[i];
    }
}

// ---------------------------------------------------------------------------
// Solver parameters
// ---------------------------------------------------------------------------

Params* or_params_new() { return new Params(); }

void or_params_delete(Params* p) { delete p; }

void or_params_set_double(Params* p, int param, double value) {
    p->SetDoubleParam(static_cast<Params::DoubleParam>(param), value);
}

double or_params_get_double(Params* p, int param) {
    return p->GetDoubleParam(static_cast<Params::DoubleParam>(param));
}

void or_params_set_integer(Params* p, int param, int value) {
    p->SetIntegerParam(static_cast<Params::IntegerParam>(param), value);
}

int or_params_get_integer(Params* p, int param) {
    return p->GetIntegerParam(static_cast<Params::IntegerParam>(param));
}

void or_params_reset(Params* p) { p->Reset(); }

void or_params_reset_double(Params* p, int param) {
    p->ResetDoubleParam(static_cast<Params::DoubleParam>(param));
}

void or_params_reset_integer(Params* p, int param) {
    p->ResetIntegerParam(static_cast<Params::IntegerParam>(param));
}

}  // extern "C"
