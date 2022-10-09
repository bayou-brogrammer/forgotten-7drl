use super::*;

#[derive(Clone)]
enum PauseMenuEntry {
    Resume,
    SaveQuit,
    Save,
    NewGame,
    Help,
    Clear,
}

pub enum PauseOutput {
    ContinueGame { running: state::Running },
    MainMenu,
    Quit,
}

fn pause_menu() -> AppCF<PauseMenuEntry> {
    use PauseMenuEntry::*;

    let mut builder = menu_builder().vi_keys();
    let mut add_item = |entry, name, ch: char| {
        let identifier = MENU_FADE_SPEC.identifier(move |b| write!(b, "({}) {}", ch, name).unwrap());
        builder.add_item_mut(item(entry, identifier).add_hotkey_char(ch));
    };

    add_item(Resume, "Resume", 'r');
    add_item(SaveQuit, "Save and Quit", 'q');
    add_item(Save, "Save", 's');
    add_item(NewGame, "New Game", 'n');
    add_item(Help, "Help", 'h');
    add_item(Clear, "Clear", 'c');
    builder.build_cf()
}

pub fn pause_menu_loop(running: state::Running) -> AppCF<PauseOutput> {
    use PauseMenuEntry::*;
    let text_width = 64;
    menu_style(pause_menu().menu_harness().repeat(running, move |running, entry_or_escape| {
        match entry_or_escape {
            Ok(entry) => match entry {
                Resume => break_(PauseOutput::ContinueGame { running }),
                SaveQuit => on_state(|state: &mut State| {
                    state.save_instance(running);
                    PauseOutput::Quit
                })
                .break_(),
                Save => on_state(|state: &mut State| PauseOutput::ContinueGame {
                    running: state.save_instance(running),
                })
                .break_(),
                NewGame => {
                    on_state(|state: &mut State| PauseOutput::ContinueGame { running: state.new_game() })
                        .break_()
                }
                Help => text::help(text_width).continue_with(running),
                Clear => on_state(|state: &mut State| {
                    state.clear_saved_game();
                    PauseOutput::MainMenu
                })
                .break_(),
            },
            Err(_escape_or_start) => break_(PauseOutput::ContinueGame { running }),
        }
    }))
}
