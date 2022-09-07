use super::*;
use crate::text;

#[derive(Clone)]
enum MainMenuEntry {
    NewGame,
    Options,
    Help,
    Prologue,
    Epilogue,
    Quit,
}

pub enum MainMenuOutput {
    NewGame { new_running: witness::Running },
    Quit,
}

fn main_menu() -> AppCF<MainMenuEntry> {
    on_state_then(|state: &mut State| {
        use MainMenuEntry::*;

        let mut builder = menu_builder().vi_keys();
        let mut add_item = |entry, name, ch: char| {
            let identifier = MENU_FADE_SPEC.identifier(move |b| write!(b, "({}) {}", ch, name).unwrap());
            builder.add_item_mut(item(entry, identifier).add_hotkey_char(ch));
        };

        add_item(NewGame, "New Game", 'n');
        add_item(Options, "Options", 'o');
        add_item(Help, "Help", 'h');
        add_item(Prologue, "Prologue", 'p');
        if state.config.won {
            add_item(Epilogue, "Epilogue", 'e');
        }
        add_item(Quit, "Quit", 'q');
        builder.build_cf()
    })
}

fn title_decorate<T: 'static>(cf: AppCF<T>) -> AppCF<T> {
    cf.with_title(styled_string("Rrouge?".to_string(), Style::plain_text()), 2).centre()
}

pub fn main_menu_loop() -> AppCF<MainMenuOutput> {
    use MainMenuEntry::*;
    title_decorate(main_menu())
        .repeat_unit(move |entry| match entry {
            NewGame => {
                on_state(|state: &mut State| MainMenuOutput::NewGame { new_running: state.new_game() })
                    .break_()
            }
            Options => title_decorate(options_menu()).continue_(),
            Help => text::help(MAIN_MENU_TEXT_WIDTH).centre().continue_(),
            Prologue => text::prologue(MAIN_MENU_TEXT_WIDTH).centre().continue_(),
            Epilogue => text::epilogue(MAIN_MENU_TEXT_WIDTH).centre().continue_(),
            Quit => val_once(MainMenuOutput::Quit).break_(),
        })
        .bound_width(42)
    // .overlay(MenuBackgroundComponent, 10)
}
