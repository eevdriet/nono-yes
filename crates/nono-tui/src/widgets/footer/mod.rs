mod actions;

use nono::{Axis, Fill};
use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect},
    style::{Color, Style},
    symbols,
    text::{Line, Span},
    widgets::{LineGauge, Paragraph, StatefulWidgetRef, Widget},
};

use crate::{AppState, PuzzleState};

#[derive(Debug)]
pub struct FooterWidget;

impl StatefulWidgetRef for &FooterWidget {
    type State = AppState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let rect = |offset: u16| -> Rect {
            Rect {
                x: area.x,
                y: area.y + offset,
                width: area.width,
                height: 1,
            }
        };

        // Progress
        self.render_colors(rect(0), buf, state);
        self.render_stats(rect(1), buf, state);
        self.render_progress(rect(2), buf, state);
    }
}

impl FooterWidget {
    fn create_fill_spans(&self, fill: Fill, state: &PuzzleState) -> Vec<Span<'_>> {
        let mut spans: Vec<Span> = Vec::new();

        let mut style = match state.fill == fill {
            true => Style::default().bold().underlined(),
            _ => Style::default(),
        };
        let color = state
            .style
            .fill_color(fill)
            .expect("Fill {fill:?} should have a defined color");

        style = style.underline_color(color);

        // Color brush itself
        let symbol = fill.symbol();
        let span = Span::styled(format!("{symbol} "), style.fg(color));
        spans.push(span);

        // Id of the color
        let key = state
            .style
            .key_from_fill(fill)
            .expect("Fill {fill:?} should define a id char");

        let span = Span::styled(key.to_string(), style.fg(Color::White));
        spans.push(span);

        spans
    }

    fn render_colors(&self, area: Rect, buf: &mut Buffer, state: &AppState) {
        // Show the available colors
        let mut spans: Vec<Span> = Vec::new();

        let fills: Vec<_> = (0..state.puzzle.style.colors.len())
            .map(|c| Fill::Color(c as u16 + 1))
            .collect();

        for fill in fills {
            spans.extend(self.create_fill_spans(fill, &state.puzzle));
            spans.push(Span::raw(" "));
        }

        let line = Line::from(spans);

        Paragraph::new(line)
            .alignment(Alignment::Center)
            .render(area, buf);

        // Show the current fill
        let fill = state.puzzle.fill;
        let symbol = fill.symbol();
        let color = state
            .puzzle
            .style
            .fill_color(fill)
            .expect("Current fill {fill:?} should have a defined color");

        Span::styled(symbol.to_string().repeat(3), Style::default().fg(color))
            .into_left_aligned_line()
            .render(area, buf);
    }

    fn render_progress(&self, area: Rect, buf: &mut Buffer, state: &AppState) {
        // Determine how many of the cells are filled (non-blank)
        let fill_count = state
            .puzzle
            .puzzle
            .iter_cells()
            .filter(|fill| !matches!(fill, Fill::Blank))
            .count() as u16;

        let fill_perc = fill_count as f64 / state.puzzle.puzzle.size() as f64;

        // let gauge = Gauge::default().ratio(fill_perc);
        let gauge = LineGauge::default()
            .filled_style(Style::new().white().on_black().bold())
            .filled_symbol(symbols::line::THICK_HORIZONTAL)
            .ratio(fill_perc);

        gauge.render(area, buf);
    }

    fn render_stats(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        // Show the current position in the puzzle and the main axis
        let cursor = state.puzzle.cursor;
        let symbol = match state.puzzle.motion_axis {
            Axis::Row => "↔",
            Axis::Col => "↕",
        };

        Span::styled(
            format!("{},{} {symbol}", cursor.y + 1, cursor.x + 1),
            Style::default().fg(Color::White),
        )
        .into_left_aligned_line()
        .render(area, buf);

        // Show the dimensions of the puzzle
        Span::styled(
            format!(
                "{},{}",
                state.puzzle.puzzle.rows(),
                state.puzzle.puzzle.cols()
            ),
            Style::default().fg(Color::White),
        )
        .into_right_aligned_line()
        .render(area, buf);
    }
}
