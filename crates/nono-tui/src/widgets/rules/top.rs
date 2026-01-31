use nono::{Fill, Line, Rule, Run};
use ratatui::{
    layout::{Alignment, Margin},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, StatefulWidgetRef, TitlePosition, Widget},
};

use crate::{AppState, Focus, run_style};

#[derive(Debug)]
pub struct ColRulesWidget {
    name: String,
    rules: Vec<Rule>,
}

impl StatefulWidgetRef for &ColRulesWidget {
    type State = AppState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let inner = area.inner(Margin::new(1, 1));

        let mut style = Style::default().fg(Color::DarkGray).dim();
        if matches!(state.focus, Focus::RulesTop) {
            style = style.fg(Color::White).not_dim().bold();
        }

        Block::new()
            .borders(Borders::TOP)
            .title(" Cols ")
            .title_alignment(Alignment::Center)
            .title_position(TitlePosition::Top)
            .border_style(style)
            .render(area, buf);

        self.draw(inner, buf, state);
    }
}

impl ColRulesWidget {
    pub fn new(name: String, rules: Vec<Rule>) -> Self {
        Self { name, rules }
    }

    fn draw(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let vp = &state.puzzle.viewport;
        let cols = state.puzzle.puzzle.cols();
        let cell_width = state.puzzle.style.cell_width as usize;

        // Keep track of the horizontal position
        let mut x = vp.area.x;

        for (c, col) in (vp.col_start..vp.col_end).enumerate() {
            let c = c as u16;
            let line = Line::Col(col);

            // Derive the runs and display a single 0 if none
            let rule = &self.rules[col as usize];
            let runs = match rule.runs().len() {
                0 => &vec![Run {
                    count: 0,
                    fill: Fill::Blank,
                }],
                _ => rule.runs(),
            };

            //
            let len = runs.len() as u16;
            let height = area.height;

            let mut y = area.y;

            for r in 0..len.min(height) {
                if y >= area.bottom() {
                    break;
                }

                if y >= area.bottom() {
                    let text = format!("{:>cell_width$}", "⋯");
                    let style = run_style(Fill::Blank, rule, line, state);

                    buf.set_string(x, y, text, style);
                    break;
                } else {
                    let run = runs[r as usize];
                    let text = format!("{:>cell_width$}", run.count);
                    let style = run_style(run.fill, rule, line, state);

                    buf.set_string(x, y, text, style);
                    y += 1;
                }
            }

            // Advance to next viewport column and skip grid dividors
            x += cell_width as u16;

            if let Some(size) = state.puzzle.style.grid_size
                && (c + 1).is_multiple_of(size)
                && c != cols - 1
            {
                x += 1;
            }
        }
    }
}
