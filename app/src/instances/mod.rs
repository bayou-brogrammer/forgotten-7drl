use super::prelude::*;
use gridbugs::chargrid::{
    border::BorderStyle,
    input::*,
    menu::{builder::*, Menu},
    text::StyledString,
};

mod main_menu;
mod options;
mod playing;
mod prologue;

pub use main_menu::*;
pub use options::*;
pub use playing::*;
pub use prologue::*;

fn _menu_style<T: 'static>(menu: AppCF<T>) -> AppCF<T> {
    menu.border(BorderStyle::default()).fill(color::MENU_BACKGROUND).centre().overlay_tint(
        render_state(|state: &State, ctx, fb| state.render(color::CURSOR, ctx, fb)),
        gridbugs::chargrid::core::TintDim(63),
        10,
    )
}
