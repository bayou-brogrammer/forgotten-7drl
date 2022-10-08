use super::*;
use gridbugs::chargrid::menu;

pub const MAIN_MENU_TEXT_WIDTH: u32 = 40;

pub const MENU_FADE_SPEC: menu::identifier::fade_spec::FadeSpec = {
    use menu::identifier::fade_spec::*;
    FadeSpec {
        on_select: Fade {
            to: To {
                bold: true,
                underline: false,
                rgba32: Layers { foreground: color::LIGHT_GREY, background: color::STRIPE },
            },
            from: From::current(),
            durations: Layers {
                foreground: Duration::from_millis(128),
                background: Duration::from_millis(128),
            },
        },
        on_deselect: Fade {
            to: To {
                bold: false,
                underline: false,
                rgba32: Layers { foreground: color::STRIPE, background: color::MENU_BACKGROUND },
            },
            from: From::current(),
            durations: Layers {
                foreground: Duration::from_millis(128),
                background: Duration::from_millis(128),
            },
        },
    }
};
