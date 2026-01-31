use crate::{
    Action, ActionInput, ActionOutcome, ActionResult, AppState, HandleAction, RulesWidget,
};

impl HandleAction for &RulesWidget {
    fn handle_command(&self, input: ActionInput, _state: &mut AppState) -> ActionResult {
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
}
