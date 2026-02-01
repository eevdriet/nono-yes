mod actions;

pub use actions::*;

use nono::{Fill, Line, LineValidation, Rule, Run};
use ratatui::{
    layout::{Alignment, Margin, Position as AppPosition},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::{Line as TextLine, Span},
    widgets::{Block, Borders, Paragraph, StatefulWidgetRef, TitlePosition, Widget},
};

use crate::{AppState, Focus, run_style, status_info};

#[derive(Debug)]
pub struct RowRulesWidget {
    name: String,
    rules: Vec<Rule>,
}

impl StatefulWidgetRef for &RowRulesWidget {
    type State = AppState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let mut style = Style::default().fg(Color::Gray).dim();
        if matches!(state.focus, Focus::RulesLeft) {
            style = style.fg(Color::White).not_dim().bold();
        }

        Block::new()
            .borders(Borders::TOP)
            .title(" Rows ")
            .border_style(style)
            .title_alignment(Alignment::Center)
            .title_position(TitlePosition::Top)
            .render(area.inner(Margin::new(1, 0)), buf);

        self.draw(area, buf, state);
    }
}

impl RowRulesWidget {
    pub fn new(name: String, rules: Vec<Rule>) -> Self {
        Self { name, rules }
    }

    fn draw(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let vp = &state.puzzle.viewport;
        let rows = state.puzzle.puzzle.rows();
        let cell_height = state.puzzle.style.cell_height;

        // Keep track of the vertical position
        let mut y = vp.area.y;

        for row in vp.row_start..vp.row_end {
            let rule = &self.rules[row as usize];
            let line = Line::Row(row);
            let validation = state.puzzle.puzzle.validate(rule, line);

            // NOTE: this breaks the coloring of the runs if status is drawn after the runs
            self.draw_status(line, &validation, y, area, buf, state);
            self.draw_runs(rule, &validation, line, y, area, buf, state);

            // Advance to next viewport row and skip grid dividors
            y += cell_height;

            if let Some(size) = state.puzzle.style.grid_size
                && (row + 1).is_multiple_of(size)
                && row != rows - 1
            {
                y += cell_height;
            }
        }
    }

    fn draw_runs(
        &self,
        rule: &Rule,
        validation: &LineValidation,
        line: Line,
        y: u16,
        area: Rect,
        buf: &mut Buffer,
        state: &AppState,
    ) {
        let cell_height = state.puzzle.style.cell_height;
        let row = line.line();

        let mut spans: Vec<Span> = Vec::new();
        let runs = match rule.runs().len() {
            0 => &vec![Run {
                count: 0,
                fill: Fill::Blank,
            }],
            _ => rule.runs(),
        };

        // Skip grid dividor row
        let mut width = 0;

        for (r, run) in runs.iter().enumerate() {
            let pos = AppPosition::new(r as u16, row);
            let text = match pos == state.rules_left.cursor {
                true => "A".to_string(),
                false => run.count.to_string(),
            };

            let len = text.len() as u16;

            // Don't overflow the area if the rule is too long to draw
            if width >= area.width {
                break;
            }
            // Instead hide the remaining runs
            else if width + len >= area.width {
                spans.push(Span::raw("⋯"));
                break;
            } else {
                // If not, draw the run
                width += len;

                let style = run_style(run.fill, rule, r as u16, line, validation, state);
                let span = Span::styled(text, style);

                spans.push(span);

                // Add a dividor to the next run if it fits
                if r != runs.len() - 1 && width < area.width {
                    spans.push(Span::raw(" "));
                    width += 1;
                }
            }
        }

        let area = Rect {
            x: area.x,
            y,
            width: area.width,
            height: cell_height,
        };
        TextLine::from(spans)
            .alignment(Alignment::Right)
            .style(Style::reset())
            .render(area.inner(Margin::new(2, 0)), buf);

        // Paragraph::new(line)
        //     .alignment(Alignment::Right) // clues hug puzzle
        //     .render(area.inner(Margin::new(2, 0)), buf);
    }

    fn draw_status(
        &self,
        line: Line,
        validation: &LineValidation,
        y: u16,
        area: Rect,
        buf: &mut Buffer,
        state: &AppState,
    ) {
        let cell_height = state.puzzle.style.cell_height;
        let (style, symbol) = status_info(line, validation, state);

        let area = Rect {
            x: area.x,
            y,
            width: area.width,
            height: cell_height,
        };

        Paragraph::new(format!("{symbol}"))
            .alignment(Alignment::Right)
            .style(style)
            .render(area, buf);
    }
}
