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
    text_component(
        width,
        vec![
            bold("Combat\n"),
            plain("Each weapon has a DMG(♥) and PEN(♦) stat, and each enemy has heatlh(♥) and armour(♦). "),
            plain(
                "If an enemy is hit with a weapon that has a higher PEN than their armour, their health is ",
            ),
            plain(
                "reduced by the weapon's DMG. If a projectile's PEN exceeds an enemy's armour, it continues ",
            ),
            plain("on its path with its PEN reduced by the enemy's armour.\n\n"),
            // Enemies
            bold("Enemies\n"),
            plain("Minibot - basic guard robot\n"),
            plain("SecBot - upgraded minibot\n"),
            plain("RoboCop - elite guard robot. Alerts nearby robots when it sees you.\n"),
            plain("Doom Bot - Kill bot. Very hard to kill. Explodes on death\n\n"),
            // Keys
            bold("Default Keyboard Controls\n"),
            plain("Movement/Aim: Arrows/WASD/HJKL\n"),
            plain("Cancel Aim: Escape\n"),
            plain("Wait: Space\n"),
            plain("Examine: X\n"),
            plain("Descend: Period\n"),
            plain("Get Weapon: G\n"),
            plain("Fire Ranged Weapon: 1-3\n\n"),
            // Gamepad
            bold("Default Gamepad Controls\n"),
            plain("Movement/Aim: D-Pad\n"),
            plain("Cancel Aim: Select\n"),
            plain("Wait: Select\n"),
            plain("Examine: Right Bumper\n"),
            plain("Descend: Left Bumper\n"),
            plain("Get Weapon: Y/Triangle\n"),
            plain("Fire Ranged Weapon Slot 1: X/Square\n"),
            plain("Fire Ranged Weapon Slot 2: A/Cross\n"),
            plain("Fire Ranged Weapon Slot 2: B/Circle\n"),
            faint("\n\n\n\n\nPress any key..."),
        ],
    )
}

pub fn epilogue1(width: u32) -> AppCF<()> {
    text_component(
        width,
        vec![
            norm(
                "As you watch the reactor explode, you feel a sense of relief. You have done your part. 
            You have saved humanity.\n\n",
            ),
            norm("You pickup your radio and radio in \"Mission Acomplished.\"....\n\n....*static*"),
            faint("\n\n\n\nPress any key..."),
        ],
    )
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
