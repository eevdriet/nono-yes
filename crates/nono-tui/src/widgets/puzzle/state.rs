use nono::{Axis, Fill, Position, Puzzle};
use ratatui::layout::{Position as AppPosition, Rect, Size};

use crate::{PuzzleStyle, Selection};

#[derive(Debug)]
pub struct PuzzleState {
    pub puzzle: Puzzle,

    pub style: PuzzleStyle,

    /// Selected area of the viewport
    pub selection: Selection,

    /// Position within the puzzle for solving
    pub cursor: AppPosition,

    /// Viewport that contains the visible area of the puzzle
    pub viewport: Rect,

    /// Offset of the puzzle with its top-left most cell
    pub scroll: Position,

    // Solving properties
    pub fill: Fill,
    pub motion_axis: Axis,
}

impl PuzzleState {
    pub fn new(puzzle: Puzzle, style: PuzzleStyle, fill: Fill) -> Self {
        Self {
            puzzle,
            style,
            fill,

            selection: Selection::empty(),
            cursor: AppPosition::default(),
            viewport: Rect::default(),
            scroll: Position::default(),
            motion_axis: Axis::default(),
        }
    }
    pub fn bounds(&self) -> Rect {
        let width = self.puzzle.cols();
        let height = self.puzzle.rows();

        Rect::new(0, 0, width, height)
    }

    pub fn screen_to_puzzle(&self, screen_pos: AppPosition) -> Option<Position> {
        let puzzle = &self.puzzle;

        // Start from the relative position to the viewport
        let mut x = screen_pos.x.checked_sub(self.viewport.x)?;
        let mut y = screen_pos.y.checked_sub(self.viewport.y)?;

        tracing::debug!("pos: {screen_pos:?} + viewport: {:?}", self.viewport);
        tracing::debug!("pos relative to viewport: {:?}", (x, y));

        let cell_width = self.style.cell_width;
        let cell_height = self.style.cell_height;

        // Remove grid dividors if set
        if let Some(grid_size) = self.style.grid_size {
            let block_w = grid_size * cell_width + 1;
            let block_h = grid_size * cell_height + 1;

            let div_x = x / block_w;
            let div_y = y / block_h;

            x -= div_x;
            y -= div_y;
        }

        // Adjust for variable cell dimensions to find the puzzle position
        let mut col = x / cell_width;
        let mut row = y / cell_height;

        tracing::debug!("col: {col:?} + row: {row:?}");
        tracing::debug!("cols: {:?} + rows: {:?}", puzzle.cols(), puzzle.rows());

        // Translate with the scroll position
        col += self.scroll.col;
        row += self.scroll.row;

        (col < puzzle.cols() && row < puzzle.rows()).then_some(Position { col, row })
    }

    pub fn puzzle_to_screen(&self, puzzle_pos: Position) -> Option<AppPosition> {
        // Start from the viewport origin
        let mut x = self.viewport.x;
        let mut y = self.viewport.y;

        // Determine the puzzle position visible within the viewport
        let col = puzzle_pos.col.checked_sub(self.scroll.col)?;
        let row = puzzle_pos.row.checked_sub(self.scroll.row)?;

        // Adjust for variable cell dimensions and add the puzzle position
        let cell_width = self.style.cell_width;
        let cell_height = self.style.cell_height;

        x += col * cell_width;
        y += row * cell_height;

        // Add grid dividors if set
        if let Some(grid_size) = self.style.grid_size {
            x += col / grid_size;
            y += row / grid_size;
        }

        Some(AppPosition::new(x, y))
    }

    pub fn visible_cells(&self) -> Size {
        let puzzle = &self.puzzle;

        let top_left = AppPosition::new(self.viewport.x, self.viewport.y);
        let bottom_right = AppPosition::new(
            self.viewport.x + self.viewport.width - 1,
            self.viewport.y + self.viewport.height - 1,
        );

        let start = self.screen_to_puzzle(top_left).unwrap_or_else(|| {
            panic!(
                "Viewport top-left {top_left:?} should be in-bounds ({} rows, {} cols)",
                puzzle.rows(),
                puzzle.cols()
            )
        });

        let end = self.screen_to_puzzle(bottom_right).unwrap_or_else(|| {
            panic!(
                "Viewport bottom-right {bottom_right:?} should be in-bounds ({} rows, {} cols)",
                puzzle.rows(),
                puzzle.cols()
            )
        });

        Size::new(end.col - start.col + 1, end.row - start.row + 1)
    }

    pub fn size(&self) -> Size {
        let cols = self.puzzle.cols();
        let rows = self.puzzle.rows();

        let (col_div_count, row_div_count) = match self.style.grid_size {
            Some(size) => ((cols - 1) / size, (rows - 1) / size),
            _ => (0, 0),
        };

        let width = cols * self.style.cell_width + col_div_count;
        let height = rows * self.style.cell_height + row_div_count;

        Size::new(width, height)
    }

    pub fn keep_cursor_visible(&mut self) {
        let (col, row) = self.cursor.into();
        let (vis_cols, vis_rows) = self.visible_cells().into();

        let scroll = &mut self.scroll;

        // Horizontal
        if col < scroll.col {
            scroll.col = col;
        } else if col >= scroll.col + vis_cols {
            scroll.col = col - vis_cols + 1;
        }

        // Vertical
        if row < scroll.row {
            scroll.row = row;
        } else if row >= scroll.row + vis_rows {
            scroll.row = row - vis_rows + 1;
        }
    }
}
