mod actions;
mod left;
mod state;
mod top;

pub use left::*;
pub use state::*;
pub use top::*;

use nono::{Fill, Line, LineValidation, Rule};
use ratatui::style::{Color, Modifier, Style};

use crate::AppState;

pub fn run_style(fill: Fill, rule: &Rule, line: Line, state: &AppState) -> Style {
    let validation = state.puzzle.puzzle.validate(rule, line);

    let color = state
        .puzzle
        .style
        .fill_color(fill)
        .expect("Fill {fill:?} should have a defined color");

    let base = Style::default().fg(color);

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

    // Highlight the active lines
    let is_active = match line {
        Line::Row(row) if state.puzzle.cursor.y == row => true,
        Line::Col(col) if state.puzzle.cursor.x == col => true,
        _ => false,
    };

    if is_active {
        style = style.add_modifier(Modifier::BOLD);
    }

    style
}
