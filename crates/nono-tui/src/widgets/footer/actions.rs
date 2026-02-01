use nono::{Fill, Result};

use crate::{
    Action, ActionInput, ActionOutcome, AppState, FooterWidget, HandleAction, MotionRange,
};

impl HandleAction for &FooterWidget {
    fn handle_motion(
        &self,
        input: ActionInput,
        state: &mut AppState,
    ) -> Result<(ActionOutcome, Option<MotionRange>)> {
        let action = input.action;

        if let Fill::Color(curr) = state.puzzle.fill {
            let next = match action {
                Action::MoveLeft | Action::ScrollLeft => (curr - 1).max(1),
                Action::MoveRight | Action::ScrollRight => {
                    (curr + 1).min(state.puzzle.style.colors.len() as u16 - 1)
                }
                _ => curr,
            };

            state.puzzle.fill = Fill::Color(next);
        }

        Ok((ActionOutcome::Consumed, None))
    }

    fn handle_command(&self, input: ActionInput, _state: &mut AppState) -> crate::ActionResult {
        let action = input.action;

        // Lose focus commands
        if matches!(
            action,
            Action::FocusDown | Action::FocusUp | Action::FocusLeft | Action::FocusRight
        ) {
            tracing::debug!("LOST FOCUS");
            return Ok(ActionOutcome::LoseFocus);
        }

        Ok(ActionOutcome::Consumed)
    }
}
