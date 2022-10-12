use crate::prelude::*;
use gridbugs::chargrid::control_flow::on_state_then;

pub fn game_over() -> AppCF<()> {
    on_state_then(move |state: &mut State| {
        state.examine_message = None;
        state.cursor = None;
        state.clear_saved_game();

        state.audio_state.loop_music(Audio::EndTextSad, state.config.music_volume);

        text::game_over(MAIN_MENU_TEXT_WIDTH)
    })
    .centre()
}
