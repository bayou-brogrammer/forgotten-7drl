use super::*;

pub const BOLD: Style = Style::new().with_foreground(color::STRIPE).with_bold(true);
pub const NORMAL: Style = Style::new().with_foreground(color::STRIPE).with_bold(false);
pub const FAINT: Style = Style::new().with_foreground(color::STRIPE).with_bold(false);
pub const PLAIN: Style = Style::new().with_foreground(Rgba32::new_grey(255));

pub fn t(text: &str, style: Style) -> StyledString {
    StyledString { string: text.to_string(), style }
}
pub fn f(text: &str) -> StyledString {
    StyledString { string: text.to_string(), style: FAINT }
}
pub fn b(text: &str) -> StyledString {
    StyledString { string: text.to_string(), style: BOLD }
}
pub fn plain(text: &str) -> StyledString {
    StyledString { string: text.to_string(), style: PLAIN }
}

fn text_component(width: u32, text: Vec<StyledString>) -> AppCF<()> {
    Text::new(text).wrap_word().cf().set_width(width).press_any_key()
}

pub fn prologue(width: u32) -> AppCF<()> {
    text_component(
        width,
        vec![
            t("You try one final time to page command center, but all you hear is static. 
            You stare at the last MRE and water bottle in the storage locker. It has been 3 weeks since the last communication with home base.\n\n
            You are the last survivor of the 3rd expedition to the planet X-23. Tasked with researching the rich mines of the X-23 seemed like an easy task
            until they showed up. Now you have to fight for your life to escape.\n\n
            You page one more time.....nothing\n\n", NORMAL),
            t("You come to the realization that ", NORMAL),
            t("You are forgotten\n\n", BOLD),
            t("\n\n\n\nPress any key...", FAINT),
        ],
    )
}

pub fn help(width: u32) -> AppCF<()> {
    text_component(width, vec![b("Combat\n")])
}

pub fn epilogue1(width: u32) -> AppCF<()> {
    text_component(width, vec![])
}

pub fn epilogue2(width: u32) -> AppCF<()> {
    text_component(width, vec![])
}

pub fn epilogue(width: u32) -> AppCF<()> {
    epilogue1(width).and_then(move |()| epilogue2(width))
}
