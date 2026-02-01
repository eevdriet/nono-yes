use nono::{Position, Result};

use crate::{
    Action, ActionInput, ActionOutcome, AppState, ColRulesWidget, Focus, HandleAction, MotionRange,
    app_to_puzzle, puzzle_to_app, widgets::rules::actions::handle_command,
};

impl HandleAction for &ColRulesWidget {
    fn handle_command(&self, input: ActionInput, state: &mut AppState) -> crate::ActionResult {
        handle_command(input, state)
    }

    fn handle_motion(
        &self,
        input: ActionInput,
        state: &mut AppState,
    ) -> Result<(ActionOutcome, MotionRange)> {
        let action = input.action;
        let count = input.repeat.unwrap_or(1);

        let is_left = matches!(state.focus, Focus::RulesLeft);
        let is_top = matches!(state.focus, Focus::RulesTop);

        let puzzle = &state.puzzle.puzzle;
        let max_row = puzzle.rows() - 1;
        let max_col = puzzle.cols() - 1;

        // Lose focus commands
        let pos: Position = app_to_puzzle(state.puzzle.cursor);
        let col = pos.col;
        let row = pos.row;

        let end = match action {
            Action::MoveLeft if is_top => Position {
                col: col.saturating_sub(count),
                ..pos
            },
            Action::MoveRight if is_top => Position {
                col: (col + count).min(max_col),
                ..pos
            },
            Action::MoveUp if is_left => Position {
                row: row.saturating_sub(count),
                ..pos
            },
            Action::MoveDown if is_left => Position {
                row: (row + count).min(max_row),
                ..pos
            },
            _ => pos,
        };

        let cursor = puzzle_to_app(end);
        state.puzzle.cursor = cursor;
        state.puzzle.keep_cursor_visible(cursor);

        Ok((ActionOutcome::Consumed, MotionRange::Single(cursor)))
    }
}
