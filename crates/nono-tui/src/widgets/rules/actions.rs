use crate::{
    Action, ActionInput, ActionOutcome, ActionResult, AppState, ColRulesWidget, HandleAction,
    RowRulesWidget,
};

fn handle_command(input: ActionInput, _state: &mut AppState) -> ActionResult {
    let action = input.action;

    // Lose focus commands
    if matches!(
        action,
        Action::FocusDown | Action::FocusUp | Action::FocusLeft | Action::FocusRight
    ) {
        tracing::debug!("LOST RULES FOCUS");
        return Ok(ActionOutcome::LoseFocus);
    }
    Ok(ActionOutcome::Consumed)
}

impl HandleAction for &RowRulesWidget {
    fn handle_command(&self, input: ActionInput, state: &mut AppState) -> ActionResult {
        handle_command(input, state)
    }
}

impl HandleAction for &ColRulesWidget {
    fn handle_command(&self, input: ActionInput, state: &mut AppState) -> ActionResult {
        handle_command(input, state)
    }
}
