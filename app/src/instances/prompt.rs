use super::*;
use gridbugs::chargrid::pad_by::Padding;

pub fn popup(string: String) -> AppCF<()> {
    popup_style(
        StyledString {
            string,
            style: Style::new().with_bold(false).with_underline(false).with_foreground(Rgba32::new_grey(255)),
        }
        .wrap_word()
        .cf()
        .bound_width(50)
        .pad_by(Padding::all(1))
        .press_any_key(),
    )
}

pub fn prompt(prompt_witness: Prompt) -> AppCF<GameState> {
    on_state_then(move |state: &mut State| {
        state.examine_message = None;
        state.cursor = None;
        popup(prompt_witness.message().to_string()).map_val(|| prompt_witness.running())
    })
}
