pub mod formula;
pub mod solver;
pub mod utils;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formula::CnfFormula;
    use crate::utils::*;

    #[test]
    fn test1() {
        let formula = utils::parse_formula_from_cnf_file("tests/sat/block0.cnf").unwrap();
        assert_eq!(formula.len(), 3);
    }
}
