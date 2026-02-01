use nono::{Position, Result, Rule};

use crate::{
    Action, ActionInput, ActionOutcome, AppState, HandleAction, MotionRange, RowRulesWidget,
    app_to_puzzle, puzzle_to_app, widgets::rules::actions::handle_command,
};

impl HandleAction for &RowRulesWidget {
    fn handle_command(&self, input: ActionInput, state: &mut AppState) -> crate::ActionResult {
        handle_command(input, state)
    }

    fn handle_motion(
        &self,
        input: ActionInput,
        state: &mut AppState,
    ) -> Result<(ActionOutcome, MotionRange)> {
        let rule_state = &mut state.rules_left;

        let action = input.action;
        let count = input.repeat.unwrap_or(1);

        // Lose focus commands
        let get_max_col = |row: u16| -> u16 {
            let rule = &rule_state.rules[row as usize];
            rule.runs().len() as u16 - 1
        };

        let next_back_idx = |col: u16, row: u16, next_row: u16| -> u16 {
            let back = get_max_col(row).saturating_sub(col);
            let next_back = get_max_col(next_row).saturating_sub(back);

            next_back
        };

        let pos: Position = app_to_puzzle(rule_state.cursor);
        let col = pos.col;
        let row = pos.row;

        let max_row = state.puzzle.puzzle.rows() - 1;
        let max_col = get_max_col(row);

        let end = match action {
            Action::MoveLeft => Position {
                col: col.saturating_sub(count),
                ..pos
            },
            Action::MoveRight => Position {
                col: (col + count).min(max_col),
                ..pos
            },
            Action::MoveUp => {
                let next_row = row.saturating_sub(count);
                let next_col = next_back_idx(col, row, next_row);

                Position {
                    row: next_row,
                    col: next_col,
                }
            }
            Action::MoveDown => {
                let next_row = (row + count).min(max_row);
                let next_col = next_back_idx(col, row, next_row);

                Position {
                    row: next_row,
                    col: next_col,
                }
            }
            _ => pos,
        };

        tracing::info!("{pos:?} -> {end:?}");

        let cursor = puzzle_to_app(end);
        rule_state.cursor = cursor;

        Ok((ActionOutcome::Consumed, MotionRange::Single(cursor)))
    }
}
