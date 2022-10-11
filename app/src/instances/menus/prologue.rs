use super::*;

pub fn first_run_prologue() -> AppCF<()> {
    on_state_then(|state: &mut State| {
        if state.config.first_run {
            state.config.first_run = false;
            state.save_config();
            crate::text::prologue(MAIN_MENU_TEXT_WIDTH).centre()
        } else {
            unit().some()
        }
    })
}
