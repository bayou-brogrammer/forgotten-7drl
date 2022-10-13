use super::prelude::*;
use gridbugs::chargrid::{
    input::*,
    menu::{builder::*, Menu},
    pad_by::Padding,
    text::{StyledString, Text},
};

mod examine;
mod gameover;
mod menus;
mod playing;
mod weapon;
mod win;

pub use examine::*;
pub use gameover::*;
pub use menus::*;
pub use playing::*;
pub use weapon::*;
pub use win::*;

pub fn _popup_delay(string: String) -> AppCF<()> {
    popup_style(
        StyledString {
            string: string.clone(),
            style: Style::new().with_bold(false).with_underline(false).with_foreground(Rgba32::new_grey(255)),
        }
        .wrap_word()
        .cf()
        .bound_width(50)
        .pad_by(Padding::all(1))
        .delay(Duration::from_secs(2)),
    )
    .then(|| popup(string))
}

pub fn popup(string: String) -> AppCF<()> {
    popup_style(
        Text::new(vec![
            StyledString {
                string,
                style: Style::new()
                    .with_bold(false)
                    .with_underline(false)
                    .with_foreground(Rgba32::new_grey(255)),
            },
            StyledString {
                string: "\n\n(press any key to continue)".to_string(),
                style: Style::new()
                    .with_bold(false)
                    .with_underline(false)
                    .with_foreground(Rgba32::new_grey(255)),
            },
        ])
        .wrap_word()
        .cf()
        .bound_width(50)
        .pad_by(Padding::all(1))
        .press_any_key(),
    )
}
