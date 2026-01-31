use derive_more::Debug;

use crate::{Puzzle, Rule};

#[derive(Debug, Clone)]
pub struct Rules {
    pub rows: Vec<Rule>,
    pub cols: Vec<Rule>,
}

impl Rules {
    pub fn new(rows: Vec<Rule>, cols: Vec<Rule>) -> Self {
        Self { rows, cols }
    }

    pub fn from_puzzle(puzzle: &Puzzle) -> Self {
        let rows = puzzle.iter_rows().map(Rule::from_fills).collect::<Vec<_>>();
        let cols = puzzle.iter_cols().map(Rule::from_fills).collect::<Vec<_>>();

        Self { rows, cols }
    }

    pub fn row(&self, r: u16) -> &Rule {
        &self.rows[r as usize]
    }
    pub fn col(&self, c: u16) -> &Rule {
        &self.cols[c as usize]
    }
}
