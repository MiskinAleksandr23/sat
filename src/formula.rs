use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SatResult {
    SAT,
    UNSAT,
    UNKNOWN, // can't prove in given time
}

/*
x > 0 means x variable in clause
x < 0 means not(x) variable in clause
x = 0 is impossible, maybe panic? /// TODO
*/
pub type Literal = i32;

/*
[Lit_1 V ... V Lit_k]
*/
pub type OrClause = Vec<Literal>;

/*
[Lit_{1, 1} V ... V Lit_{1, k}] ∧ ... ∧ [Lit_{m, 1} V ... V Lit_{m, n}]
*/
pub type CnfFormula = Vec<OrClause>;

// must be > 0
pub type Variable = usize;

/*
positive -> true,
negative -> false,
0 -> unassigned
*/
pub type AssignedVariable = i32;
