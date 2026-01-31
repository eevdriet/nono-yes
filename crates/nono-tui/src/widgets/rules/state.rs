use nono::Rule;
use ratatui::layout::{Direction, Position as AppPosition, Rect};

use crate::{RuleDisplay, Selection};

#[derive(Debug, Default)]
pub struct RuleState {
    pub rules: Vec<Rule>,

    pub display: RuleDisplay,

    pub direction: Direction,

    pub cursor: AppPosition,

    pub selection: Selection,

    pub area: Rect,
}

impl RuleState {
    pub fn new(rules: Vec<Rule>, direction: Direction) -> Self {
        Self {
            rules,
            direction,
            display: RuleDisplay::default(),
            cursor: AppPosition::default(),
            selection: Selection::empty(),
            area: Rect::default(),
        }
    }

    pub fn height(&self) -> u16 {
        let heights: Vec<_> = self
            .rules
            .iter()
            .map(|rule: &Rule| rule.runs().len() as u16)
            .collect();

        match self.display {
            RuleDisplay::Auto => median(heights),
            _ => heights.iter().max().copied().unwrap_or_default(),
        }
    }

    pub fn width(&self) -> u16 {
        let rows: Vec<_> = self
            .rules
            .iter()
            .map(|rule: &Rule| {
                let runs = rule.runs();

                runs.len() as u16 - 1
                    + runs
                        .iter()
                        .map(|run| run.count.to_string().len() as u16)
                        .sum::<u16>()
            })
            .collect();

        rows.iter().max().copied().unwrap_or_default()
    }

    // pub fn size(&self) -> Size {
    //     });
    //     let heights = self.rules.iter().map(|rule: &Rule| rule.runs.len() as u16);
    //
    //     let (width, height) = match self.display {
    //         RuleDisplay::Auto => {
    //             let widths: Vec<_> = widths.collect();
    //             let heights: Vec<_> = heights.collect();
    //
    //             (median(widths), median(heights))
    //         }
    //         RuleDisplay::TryMax => (
    //             widths.max().unwrap_or_default(),
    //             heights.max().unwrap_or_default(),
    //         ),
    //     };
    //
    //     Size::new(width, height)
    // }
}

fn median(nums: Vec<u16>) -> u16 {
    let mut nums = nums;
    nums.sort();
    let mid = nums.len() / 2;

    nums[mid]
}
