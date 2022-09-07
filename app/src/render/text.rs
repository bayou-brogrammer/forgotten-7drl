use super::*;

pub const BOLD: Style = Style::new().with_foreground(color::STRIPE).with_bold(true);
pub const NORMAL: Style = Style::new().with_foreground(color::STRIPE).with_bold(false);
pub const FAINT: Style = Style::new().with_foreground(color::STRIPE).with_bold(false);

pub fn t(text: &str, style: Style) -> StyledString {
    StyledString { string: text.to_string(), style }
}
pub fn f(text: &str) -> StyledString {
    StyledString { string: text.to_string(), style: FAINT }
}
pub fn b(text: &str) -> StyledString {
    StyledString { string: text.to_string(), style: BOLD }
}

fn text_component(width: u32, text: Vec<StyledString>) -> AppCF<()> {
    Text::new(text).wrap_word().cf().set_width(width).press_any_key()
}

pub fn prologue(width: u32) -> AppCF<()> {
    text_component(
        width,
        vec![
            t("You tape over the flashing warning light. An overheating engine is the least of your worries. \
            Gotta focus.\n\n\
            The space station looms ahead. It's out of fuel, and about to come crashing down to Earth. \
            Unless you get to it first. \
            Special delivery: 1 hydrogen fuel cell with enough juice to kick the station out of this pesky \
            atmosphere and back into space where it belongs.\n\n\
            Back home your buddies are placing bets on whether you'll make it back alive. \
            Last you heard, odds were 5 to 1 against.\n\n\
            \"Docking complete,\" sounds a lifeless mechanical voice. No word yet from the station crew. Comms must be down. Figures. \
            Shouldering your pack containing the fuel cell, you trudge into the airlock. \
            Gotta lug this thing down the five flights of stairs to the fuel bay. Who designed this place?\n\n\
            A dim light flickers on in the airlock revealing words smeared in blood on the opposite door:\n", NORMAL),
            t("DON'T OPEN! DEAD INSIDE!", BOLD),
            t("\n\n\
                Better make those odds 6 to 1...", NORMAL),
                t("\n\n\n\n\n\nPress any key...", FAINT),
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
