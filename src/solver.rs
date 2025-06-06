#[allow(non_snake_case)]
use crate::formula::{AssignedVariable, CnfFormula, Literal, SatResult, Variable};
use std::collections::{HashMap, VecDeque};

pub struct DpllSolver {
    num_vars: usize,
    assinedValue: Vec<i32>,
    appearenceInClauses: Vec<(usize, usize)>,
    decisionVars: VecDeque<AssignedVariable>,
    activeVariables: Vec<usize>,
    unanssignedVariables: usize,
    formula: CnfFormula,
}

#[allow(dead_code)]
impl DpllSolver {
    fn new(num_vars: usize) -> Self {
        Self {
            num_vars,
            assinedValue: vec![0; num_vars + 1],
            decisionVars: VecDeque::new(),
            formula: CnfFormula::new(),
            appearenceInClauses: Vec::new(),
            activeVariables: Vec::new(),
            unanssignedVariables: 0,
        }
    }

    fn dpll_solve(&mut self) -> SatResult {
        loop {
            let _local_scope = self.pure_literal_elimination() + self.unit_propagation();
            if self.unanssignedVariables == 0 {
                return match self.check_sat() {
                    true => SatResult::SAT,
                    false => SatResult::UNSAT,
                };
            }
            let variable = self.find_next_variable_to_set();
            assert!(variable != 0);

            // find how many vars was assigned
            self.set_and_recalculate_params(variable, true);
            if self.dpll_solve() == SatResult::SAT {
                return SatResult::SAT;
            }
            self.backtrack(1 /* */); // TODO

            self.set_and_recalculate_params(variable, false);
            if self.dpll_solve() == SatResult::SAT {
                return SatResult::SAT;
            }
            return SatResult::UNSAT;
        }
    }

    fn find_next_variable_to_set(&self) -> Variable {
        for i in 1..=self.num_vars {
            if self.assinedValue[i] == 0 {
                return i;
            }
        }
        return 0;
    }

    fn set_and_recalculate_params(&mut self, var: Variable, value: bool) -> bool {
        let mut assigned: i32 = var as i32;
        if !value {
            assigned *= -1
        };
        self.decisionVars.push_back(assigned);
        self.assinedValue[var] = if value { 1 } else { -1 };

        true
    }

    fn pure_literal_elimination(&mut self) -> usize {
        let mut total_eliminations = 0;
        loop {
            let mut local_eliminations = 0;
            for i in 1..=self.num_vars {
                let (app_pos, app_neg) = self.appearenceInClauses[i];
                if app_pos > 0 && app_neg == 0 {
                    self.set_and_recalculate_params(i, true);
                    local_eliminations += 1;
                    continue;
                } else if app_neg > 0 && app_pos == 0 {
                    self.set_and_recalculate_params(i, false);
                    local_eliminations += 1;
                    continue;
                }
            }
            total_eliminations += local_eliminations;
            if local_eliminations == 0 {
                break;
            }
        }
        return total_eliminations;
    }

    fn calculate_statistics() {
        todo!();
    }

    #[inline]
    fn all_variables_set(&self) -> bool {
        return self.decisionVars.len() == self.num_vars;
    }

    fn unit_propagation(&self) -> usize {
        let mut gloval_updates = 0;
        loop {
            let mut local_updates = 0;
            for i in 0..self.formula.len() {
                if self.activeVariables[i] == 1 {
                    let _needed_variable = *self.formula[i]
                        .iter()
                        .find(|x| self.assinedValue[x.abs() as usize] == 0)
                        .unwrap();

                    //self.setAndRecalculateParams(i, i)
                    local_updates += 1;
                }
                //self.propogate()
            }
            gloval_updates += local_updates;
            if local_updates == 0 {
                break;
            }
        }
        gloval_updates
    }

    fn check_sat(&self) -> bool {
        todo!()
    }

    fn backtrack(&mut self, _count: usize) -> bool {
        todo!();
    }
}
