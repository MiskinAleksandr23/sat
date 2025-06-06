#[allow(non_snake_case)]
use crate::formula::{AssignedVariable, CnfFormula, Literal, SatResult, Variable};
use std::collections::VecDeque;

pub struct DpllSolver {
    num_vars: usize,
    assigned_values: Vec<i32>,
    appearance_in_clauses: Vec<(usize, usize)>,
    decision_vars: VecDeque<i32>,
    active_variables: Vec<usize>,
    unassigned_variables: usize,
    formula: CnfFormula,
}

#[allow(dead_code)]
impl DpllSolver {
    pub fn new(formula: CnfFormula, num_vars: usize) -> Self {
        let mut appearance_in_clauses = vec![(0, 0); num_vars + 1];

        for clause in &formula {
            for lit in clause {
                let var = lit.abs() as usize;
                if *lit > 0 {
                    appearance_in_clauses[var].0 += 1;
                } else {
                    appearance_in_clauses[var].1 += 1;
                }
            }
        }

        let f_len = formula.len();
        Self {
            num_vars,
            assigned_values: vec![0; num_vars + 1],
            decision_vars: VecDeque::new(),
            formula,
            appearance_in_clauses,
            active_variables: vec![1; f_len],
            unassigned_variables: num_vars,
        }
    }

    pub fn dpll_solve(&mut self) -> SatResult {
        let _ = self.pure_literal_elimination();
        let _ = self.unit_propagation();

        if self.unassigned_variables == 0 {
            return if self.check_sat() {
                SatResult::SAT
            } else {
                SatResult::UNSAT
            };
        }

        let variable = self.find_next_variable_to_set();
        if variable == 0 {
            return SatResult::UNSAT;
        }

        self.set_and_recalculate_params(variable, true);
        if self.dpll_solve() == SatResult::SAT {
            return SatResult::SAT;
        }
        self.backtrack(1);

        self.set_and_recalculate_params(variable, false);
        if self.dpll_solve() == SatResult::SAT {
            return SatResult::SAT;
        }
        self.backtrack(1);

        SatResult::UNSAT
    }

    fn find_next_variable_to_set(&self) -> Variable {
        (1..=self.num_vars)
            .find(|&i| self.assigned_values[i] == 0)
            .unwrap_or(0)
    }

    fn set_and_recalculate_params(&mut self, var: Variable, value: bool) {
        if self.assigned_values[var] == 0 {
            let assigned = if value { var as i32 } else { -(var as i32) };
            self.decision_vars.push_back(assigned);
            self.assigned_values[var] = if value { 1 } else { -1 };

            if self.unassigned_variables > 0 {
                self.unassigned_variables -= 1;
            }

            for (i, clause) in self.formula.iter().enumerate() {
                if self.active_variables[i] == 1 {
                    for lit in clause {
                        if lit.abs() as usize == var {
                            self.active_variables[i] = 0;
                            break;
                        }
                    }
                }
            }
        }
    }

    fn pure_literal_elimination(&mut self) -> usize {
        let mut total_eliminations = 0;

        loop {
            let mut local_eliminations = 0;

            for i in 1..=self.num_vars {
                if self.assigned_values[i] != 0 {
                    continue;
                }

                let (app_pos, app_neg) = self.appearance_in_clauses[i];
                match (app_pos > 0, app_neg > 0) {
                    (true, false) => {
                        self.set_and_recalculate_params(i, true);
                        local_eliminations += 1;
                    }
                    (false, true) => {
                        self.set_and_recalculate_params(i, false);
                        local_eliminations += 1;
                    }
                    _ => {}
                }
            }

            if local_eliminations == 0 {
                break;
            }
            total_eliminations += local_eliminations;
        }

        total_eliminations
    }

    fn unit_propagation(&mut self) -> usize {
        let mut global_updates = 0;

        loop {
            let mut local_updates = 0;

            for i in 0..self.formula.len() {
                if self.active_variables[i] == 0 {
                    continue;
                }

                let mut unassigned_lits = Vec::new();
                for lit in &self.formula[i] {
                    if self.assigned_values[lit.abs() as usize] == 0 {
                        unassigned_lits.push(*lit);
                    }
                }

                if unassigned_lits.len() == 1 {
                    let lit = unassigned_lits[0];
                    self.set_and_recalculate_params(lit.abs() as usize, lit > 0);
                    local_updates += 1;
                }
            }

            if local_updates == 0 {
                break;
            }
            global_updates += local_updates;
        }

        global_updates
    }

    fn check_sat(&self) -> bool {
        self.formula.iter().all(|clause| {
            clause.iter().any(|lit| {
                let var = lit.abs() as usize;
                (self.assigned_values[var] > 0 && *lit > 0) ||
                    (self.assigned_values[var] < 0 && *lit < 0)
            })
        })
    }

    fn backtrack(&mut self, count: usize) {
        for _ in 0..count {
            if let Some(lit) = self.decision_vars.pop_back() {
                let var = lit.abs() as usize;
                if self.assigned_values[var] != 0 {
                    self.assigned_values[var] = 0;
                    self.unassigned_variables += 1;

                    for (i, clause) in self.formula.iter().enumerate() {
                        if self.active_variables[i] == 0 && clause.iter().any(|lit| lit.abs() as usize == var) {
                            self.active_variables[i] = 1;
                        }
                    }
                }
            }
        }
    }
}