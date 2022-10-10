use gridbugs::chargrid::menu;

use super::*;

pub fn prompt(prompt_witness: Prompt) -> AppCF<GameState> {
    on_state_then(move |state: &mut State| {
        state.examine_message = None;
        state.cursor = None;
        popup(prompt_witness.message().to_string()).map_val(|| prompt_witness.running())
    })
}

fn yes_no_menu() -> AppCF<bool> {
    use menu::builder::*;
    menu_builder()
        .vi_keys()
        .add_item(
            item(true, MENU_FADE_SPEC.identifier(move |b| write!(b, "(y) Yes").unwrap()))
                .add_hotkey_char('y')
                .add_hotkey_char('Y'),
        )
        .add_item(
            item(false, MENU_FADE_SPEC.identifier(move |b| write!(b, "(n) No").unwrap()))
                .add_hotkey_char('n')
                .add_hotkey_char('N'),
        )
        .build_cf()
}

pub fn yes_no(message: String) -> AppCF<bool> {
    menu_style(
        yes_no_menu().with_title(
            cf(StyledString { string: message, style: Style::plain_text() }.wrap_word())
                .ignore_state()
                .bound_width(40),
            1,
        ),
    )
}
