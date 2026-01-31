use nono::{Fill, Line, Rule, Run};
use ratatui::{
    layout::{Alignment, Margin},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::{Line as TextLine, Span},
    widgets::{Block, Borders, Paragraph, StatefulWidgetRef, TitlePosition, Widget},
};

use crate::{AppState, Focus, run_style};

#[derive(Debug)]
pub struct RowRulesWidget {
    name: String,
    rules: Vec<Rule>,
}

impl StatefulWidgetRef for &RowRulesWidget {
    type State = AppState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let inner = area.inner(Margin::new(1, 1));

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

        self.draw(inner, buf, state);
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

            for run in runs.iter() {
                let text = run.count.to_string();
                let len = text.len() as u16;

                // Don't overflow the area if the rule is too long to draw
                if width >= area.width {
                    break;
                }
                // Instead hide the remaining runs
                else if width + len >= area.width {
                    spans.push(Span::raw("⋯"));
                    break;
                }

                // If not, draw the run
                width += len;

                let style = run_style(run.fill, rule, line, state);
                let span = Span::styled(text, style);

                spans.push(span);

                // Add a dividor to the next run if it fits
                if width < area.width {
                    spans.push(Span::raw(" "));
                    width += 1;
                }
            }

            let line = TextLine::from(spans);
            let area = Rect {
                x: area.x,
                y,
                width: area.width,
                height: cell_height,
            };

            Paragraph::new(line)
                .alignment(Alignment::Right) // clues hug puzzle
                .render(area, buf);

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
}
