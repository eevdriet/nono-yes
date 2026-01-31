use std::collections::VecDeque;

use crate::{Line, Puzzle, Result};

#[derive(Debug, Default)]
pub struct Solver {
    frontier: VecDeque<Line>,
}

impl Solver {
    pub fn solve(&mut self, _puzzle: &mut Puzzle) -> Result<bool> {
        Ok(true)
    }
}
