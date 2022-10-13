use crate::prelude::*;

pub fn win() -> AppCF<()> {
    on_state_then(move |state: &mut State| {
        state.clear_saved_game();
        state.config.won = true;
        state.save_config();
        state.audio_state.loop_music(Audio::EndTextHappy, state.config.music_volume);
        text::epilogue1(MAIN_MENU_TEXT_WIDTH)
    })
    .centre()
}
