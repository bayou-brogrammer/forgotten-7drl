use super::*;

pub const BOLD: Style = Style::new().with_foreground(color::STRIPE).with_bold(true);
pub const NORMAL: Style = Style::new().with_foreground(color::STRIPE).with_bold(false);
pub const FAINT: Style = Style::new().with_foreground(color::STRIPE).with_bold(false);
pub const PLAIN: Style = Style::new().with_foreground(Rgba32::new_grey(255));

pub fn t(text: &str, style: Style) -> StyledString {
    StyledString { string: text.to_string(), style }
}
pub fn faint(text: &str) -> StyledString {
    StyledString { string: text.to_string(), style: FAINT }
}
pub fn bold(text: &str) -> StyledString {
    StyledString { string: text.to_string(), style: BOLD }
}
pub fn plain(text: &str) -> StyledString {
    StyledString { string: text.to_string(), style: PLAIN }
}
pub fn norm(text: &str) -> StyledString {
    StyledString { string: text.to_string(), style: NORMAL }
}

fn text_component(width: u32, text: Vec<StyledString>) -> AppCF<()> {
    Text::new(text).wrap_word().cf().set_width(width).press_any_key()
}

pub fn prologue(width: u32) -> AppCF<()> {
    text_component(
        width,
        vec![
            norm("You try one final time to page command center, but all you hear is static. 
            You stare at the last MRE and water bottle in the storage locker. It has been 3 weeks since the last communication with home base.\n\n
            You are the last survivor of the team tasked to destroy the core reactor powering the robots. It has
            been 3 long years at war. They have been relentless in their pursuit of the last human resistance.
            This is the last resort. If you fail, so does humanity\n\n
            You page one more time.....nothing\n\n"),
            bold("You are forgotten\n\n"),
            faint("\n\n\n\nPress any key..."),
        ],
    )
}

pub fn help(width: u32) -> AppCF<()> {
    text_component(width, vec![bold("Combat\n")])
}

pub fn epilogue1(width: u32) -> AppCF<()> {
    text_component(width, vec![])
}

pub fn epilogue(width: u32) -> AppCF<()> {
    epilogue1(width)
}

pub fn game_over(width: u32) -> AppCF<()> {
    text_component(
        width,
        vec![
            norm("The light goes out on your head sensor. You have been defeated. "),
            norm("Even though you have been forgotten, You tried your best to destroy the reactor."),
            faint("\n\n\n\nPress any key..."),
        ],
    )
}
