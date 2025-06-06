use std::fmt::Error;
use std::fs;
use std::io::{self};
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use crate::formula::CnfFormula;

pub fn parse_formula_from_cnf_file<P: AsRef<Path>>(path: P) -> Result<CnfFormula, io::Error> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut clauses: Vec<Vec<i32>> = Vec::new();
    let mut header_found = false;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('c') {
            continue;
        }

        if line.starts_with("p") {
            if header_found {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "parse error",
                ));
            }
            header_found = true;
        } else {
            let mut clause: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            if clause.is_empty() {
                continue;
            }
            if *clause.last().unwrap() != 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "parse error",
                ));
            }
            clause.pop();
            clauses.push(clause);
        }
    }
    return Ok(clauses);
}
