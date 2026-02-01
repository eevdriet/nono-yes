use crate::{Action, ActionInput, ActionOutcome, ActionResult, AppState};

pub fn handle_command(input: ActionInput, _state: &mut AppState) -> ActionResult {
    let action = input.action;

    if matches!(
        action,
        Action::FocusDown | Action::FocusUp | Action::FocusLeft | Action::FocusRight
    ) {
        tracing::debug!("LOST RULES FOCUS");
        return Ok(ActionOutcome::LoseFocus);
    }

    Ok(ActionOutcome::Consumed)
}
