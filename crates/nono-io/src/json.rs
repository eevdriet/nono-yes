use std::path::Path;

use nono::{Fill, Nonogram, Puzzle, Rule, Rules, Run};
use serde::Deserialize;

use crate::{PuzzleLoader, error::Result};

pub struct JsonLoader;

impl PuzzleLoader for JsonLoader {
    fn load_nonogram(path: &Path) -> Result<Nonogram> {
        let text = std::fs::read_to_string(path)?;
        let raw: JsonNonogram = serde_json::from_str(&text)?;
        let nonogram = raw.try_into()?;

        Ok(nonogram)
    }
}

#[derive(Debug, Deserialize)]
pub struct JsonNonogram {
    pub colors: Vec<(u8, u8, u8)>,

    pub rows: u16,
    pub cols: u16,

    pub row_rules: Vec<Vec<RawRun>>,
    pub col_rules: Vec<Vec<RawRun>>,

    #[serde(default)]
    pub puzzle: Vec<Vec<u16>>,
}

#[derive(Debug, Deserialize)]
pub struct RawRun {
    pub fill: u16,
    pub count: u16,
}

impl TryFrom<JsonNonogram> for Nonogram {
    type Error = nono::Error;

    fn try_from(data: JsonNonogram) -> core::result::Result<Self, Self::Error> {
        let fills: Vec<_> = data
            .puzzle
            .iter()
            .flatten()
            .map(|id| Fill::Color(*id))
            .collect();

        // let puzzle = Puzzle::empty(data.rows, data.cols);
        let puzzle = if data.puzzle.is_empty() {
            Puzzle::empty(data.rows, data.cols)
        } else {
            Puzzle::new(data.rows, data.cols, fills)?
        };

        let rows: Vec<_> = data
            .row_rules
            .into_iter()
            .map(|raw| {
                let runs: Vec<_> = raw
                    .iter()
                    .map(|run| Run::new(Fill::Color(run.fill), run.count))
                    .collect();

                let mut rule = Rule::new(runs, data.cols);
                rule.generate_constraints();

                rule
            })
            .collect();

        let cols: Vec<_> = data
            .col_rules
            .into_iter()
            .map(|raw| {
                let runs: Vec<_> = raw
                    .iter()
                    .map(|run| Run::new(Fill::Color(run.fill), run.count))
                    .collect();

                let mut rule = Rule::new(runs, data.rows);
                rule.generate_constraints();

                rule
            })
            .collect();

        let rules = Rules::new(rows, cols);

        Ok(Nonogram {
            puzzle,
            rules,
            colors: data.colors,
        })
    }
}
