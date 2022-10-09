use super::*;

pub fn prompt(prompt_witness: Prompt) -> AppCF<GameState> {
    on_state_then(move |state: &mut State| {
        state.examine_message = None;
        state.cursor = None;
        popup(prompt_witness.message().to_string()).map_val(|| prompt_witness.running())
    })
}
