pub mod formula;
pub mod solver;
pub mod utils;

use utils::*;
use solver::*;
use formula::*;

#[cfg(test)]
mod solver_tests {
    use super::*;
    use crate::formula::CnfFormula;
    use crate::solver::DpllSolver;
    use tempfile::NamedTempFile;
    use std::io::Write;

    fn create_temp_cnf(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", content).unwrap();
        file
    }

    #[test]
    fn test_simple_sat() {
        let content = r"
        p cnf 2 2
        1 0
        -1 2 0
        ";
        let file = create_temp_cnf(content);

        let formula = parse_formula_from_cnf_file(file.path()).unwrap();
        let mut solver = DpllSolver::new(formula, 2);
        assert_eq!(solver.dpll_solve(), SatResult::SAT);
    }

    #[test]
    fn test_simple_unsat() {
        let content = r"
        p cnf 1 2
        1 0
        -1 0
        ";
        let file = create_temp_cnf(content);

        let formula = parse_formula_from_cnf_file(file.path()).unwrap();
        let mut solver = DpllSolver::new(formula, 1);
        assert_eq!(solver.dpll_solve(), SatResult::UNSAT);
    }

    #[test]
    fn test_pure_literal_elimination() {
        let content = r"
        p cnf 2 2
        1 2 0
        -2 0
        ";
        let file = create_temp_cnf(content);

        let formula = parse_formula_from_cnf_file(file.path()).unwrap();
        let mut solver = DpllSolver::new(formula, 2);
        assert_eq!(solver.dpll_solve(), SatResult::SAT);
    }

    #[test]
    fn test_unit_propagation() {
        let content = r"
        p cnf 3 3
        1 0
        -1 2 0
        -2 3 0
        ";
        let file = create_temp_cnf(content);

        let formula = parse_formula_from_cnf_file(file.path()).unwrap();
        let mut solver = DpllSolver::new(formula, 3);
        assert_eq!(solver.dpll_solve(), SatResult::SAT);
    }

    #[test]
    fn test_contradiction_after_propagation() {
        let content = r"
        p cnf 2 3
        1 0
        -1 2 0
        -1 -2 0
        ";
        let file = create_temp_cnf(content);

        let formula = parse_formula_from_cnf_file(file.path()).unwrap();
        let mut solver = DpllSolver::new(formula, 2);
        assert_eq!(solver.dpll_solve(), SatResult::UNSAT);
    }

    #[test]
    fn test_backtracking() {
        let content = r"
        p cnf 3 4
        1 2 0
        1 -2 0
        -1 3 0
        -1 -3 0
        ";
        let file = create_temp_cnf(content);

        let formula = parse_formula_from_cnf_file(file.path()).unwrap();
        let mut solver = DpllSolver::new(formula, 3);
        assert_eq!(solver.dpll_solve(), SatResult::UNSAT);
    }

    #[test]
    fn test_empty_formula() {
        let content = "p cnf 0 0";
        let file = create_temp_cnf(content);

        let formula = parse_formula_from_cnf_file(file.path()).unwrap();
        let mut solver = DpllSolver::new(formula, 0);
        assert_eq!(solver.dpll_solve(), SatResult::SAT);
    }

    #[test]
    fn test_empty_clause() {
        let content = r"
        p cnf 2 2
        0
        1 2 0
        ";
        let file = create_temp_cnf(content);

        let formula = parse_formula_from_cnf_file(file.path()).unwrap();
        let mut solver = DpllSolver::new(formula, 2);
        assert_eq!(solver.dpll_solve(), SatResult::UNSAT);
    }
}