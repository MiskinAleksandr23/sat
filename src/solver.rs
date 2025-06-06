#[allow(non_snake_case)]

use crate::formula::{AssignedVariable, CnfFormula, Literal, SatResult, Variable};
use std::{
    collections::{HashMap, VecDeque},
    env::consts::FAMILY,
};

pub struct DpllSolver {
    // actually, just state of solver
    numVars: usize,
    alreadyAssigned: Vec<i32>,
    appearenceInClauses: Vec<(usize, usize)>,
    decisionVars: VecDeque<AssignedVariable>,
    activeVariables: Vec<usize>,
    unanssignedVariables: usize,
    formula: CnfFormula,
}

impl DpllSolver {
    fn new(numVars: usize) -> Self {
        Self {
            numVars,
            alreadyAssigned: vec![0; numVars + 1],
            decisionVars: VecDeque::new(),
            formula: CnfFormula::new(),
            appearenceInClauses: Vec::new(),
            activeVariables: Vec::new(),
            unanssignedVariables: 0,
        }
    }

    fn solve(&mut self) -> SatResult {
        loop {
            let local_scope = self.pureLiteralElimination() + self.unitPropagation();
            if self.unanssignedVariables == 0 {
                return match self.check() {
                    true => SatResult::SAT,
                    false => SatResult::UNSAT,
                };
            }
            let variable = self.findNextVariableToSet();
            assert!(variable != 0);

            // find how many vars was assigned
            self.setAndRecalculateParams(variable, true);
            if self.solve() == SatResult::SAT {
                return SatResult::SAT;
            }
            self.backtrack(1 /* */); // TODO

            self.setAndRecalculateParams(variable, false);
            if self.solve() == SatResult::SAT {
                return SatResult::SAT;
            }
            return SatResult::UNSAT;
        }
    }

    #[allow(non_snake_case)]
    fn findNextVariableToSet(&self) -> Variable {
        for i in 1..=self.numVars {
            if self.alreadyAssigned[i] == 0 {
                return i;
            }
        }
        return 0;
    }

    #[allow(non_snake_case)]
    fn setAndRecalculateParams(&mut self, var: Variable, value: bool) -> bool {
        let mut assigned: i32 = var as i32;
        if !value {
            assigned *= -1
        };
        self.decisionVars.push_back(assigned);
        self.alreadyAssigned[var] = if value { 1 } else { -1 };

        true
    }

    fn pureLiteralElimination(&mut self) -> usize {
        let mut total_eliminations = 0;
        loop {
            let mut local_eliminations = 0;
            for i in 1..=self.numVars {
                let (app_pos, app_neg) = self.appearenceInClauses[i];
                if app_pos > 0 && app_neg == 0 {
                    self.setAndRecalculateParams(i, true);
                    local_eliminations += 1;
                    continue;
                } else if app_neg > 0 && app_pos == 0 {
                    self.setAndRecalculateParams(i, false);
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

    fn calculateStatistics() {
        todo!();
    }

    fn allSatisfied(&self) -> bool {
        return self.decisionVars.len() == self.numVars;
    }

    fn unitPropagation(&self) -> usize {
        let mut gloval_updates = 0;
        loop {
            let mut local_updates = 0;
            for i in 0..self.formula.len() {
                if self.activeVariables[i] == 1 {
                    let neededVariable = *self.formula[i]
                        .iter()
                        .find(|x| self.alreadyAssigned[x.abs() as usize] == 0)
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

    fn check(&self) -> bool {
        todo!()
    }

    fn backtrack(&mut self, count: usize) -> bool {
        todo!();
    }
}


