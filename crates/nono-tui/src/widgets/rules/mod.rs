mod actions;
mod left;
mod state;
mod top;

pub use left::*;
pub use state::*;
pub use top::*;

use nono::{Fill, Line, LineValidation, Rule};
use ratatui::{
    layout::Position,
    style::{Color, Modifier, Style},
};

use crate::{AppState, Focus};

pub fn run_style(
    fill: Fill,
    rule: &Rule,
    idx: u16,
    line: Line,
    validation: &LineValidation,
    state: &AppState,
) -> Style {
    let color = state
        .puzzle
        .style
        .fill_color(fill)
        .expect("Fill {fill:?} should have a defined color");

    let base = Style::default().fg(color).dim();

    let mut style = match validation {
        // Cross out solved lines
        LineValidation::Solved => base
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM | Modifier::CROSSED_OUT),

        // Shade invalid rules in red
        LineValidation::Invalid => base
            .fg(Color::Red)
            .add_modifier(Modifier::UNDERLINED | Modifier::BOLD),

        _ => base,
    };

    let is_active = match state.focus {
        // Highlight all runs in the active row/column
        Focus::Puzzle => {
            let cursor = state.puzzle.cursor;

            match line {
                Line::Row(row) => cursor.y == row,
                Line::Col(col) => cursor.x == col,
            }
        }
        // Highlight the cursor run in the active line
        Focus::RulesLeft => {
            let cursor = state.rules_left.cursor;

            match line {
                Line::Row(row) => Position::new(idx, row) == cursor,
                Line::Col(col) => Position::new(col, idx) == cursor,
            }
        }
        // Highlight the cursor run in the active line
        Focus::RulesTop => {
            let cursor = state.rules_top.cursor;

            match line {
                Line::Row(row) => Position::new(idx, row) == cursor,
                Line::Col(col) => Position::new(col, idx) == cursor,
            }
        }
        _ => false,
    };

    if is_active {
        style = style.add_modifier(Modifier::BOLD).not_dim();
    }

    style
}

pub fn status_info(line: Line, validation: &LineValidation, state: &AppState) -> (Style, char) {
    let cursor = state.puzzle.cursor;
    let base = Style::default().fg(Color::White);

    let mut style = match validation {
        LineValidation::Solved => base.fg(Color::Green),
        val if !val.is_valid() => base.fg(Color::Red),
        _ => base,
    };

    let is_active = match line {
        Line::Row(row) => {
            cursor.y == row && matches!(state.focus, Focus::Puzzle | Focus::RulesLeft)
        }
        Line::Col(col) => cursor.x == col && matches!(state.focus, Focus::Puzzle | Focus::RulesTop),
    };

    if is_active {
        style = style.add_modifier(Modifier::BOLD).not_dim();
    }

    let mut symbol = validation.symbol();
    if symbol == ' ' && is_active {
        symbol = match line {
            Line::Row(_) => '<',
            Line::Col(_) => '^',
        }
    };

    (style, symbol)
}
