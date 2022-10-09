use crate::prelude::*;
use gridbugs::chargrid::border::BorderStyle;
use gridbugs::chargrid::text::{StyledString, Text};

mod log;
mod queries;

pub use self::log::*;
pub use self::queries::*;

pub struct Hud {}

impl Component for Hud {
    type Output = ();
    type State = StateScope;

    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        let player = state.player();
        let player_info = state.player_info();

        let stunned_txt = if player_info.stunned { "~Stunned~" } else { "" };
        let text = vec![
            plain("Health: "),
            StyledString {
                string: format!("{}/{}", player_info.hit_points.current, player_info.hit_points.max),
                style: Style::new().with_foreground(color::HEALTH).with_bold(true),
            },
            plain("\n"),
            plain("Credit: "),
            StyledString {
                string: format!("${}", player.credit),
                style: Style::new().with_foreground(color::CREDIT_FOREGROUND).with_bold(true),
            },
            plain("\n"),
            plain("\n"),
            plain(stunned_txt),
        ];
        Text::from(text).render(&(), ctx, fb);
    }

    fn update(&mut self, _: &mut Self::State, _: Ctx, _: Event) -> Self::Output {}

    fn size(&self, _: &Self::State, ctx: Ctx) -> Size {
        println!("{:?}", ctx.bounding_box.size());
        Size::new(ctx.bounding_box.size().width() + 1, GAME_VIEW_SIZE.height() - 2)
    }
}

pub fn render_hud(scope: &StateScope, ctx: Ctx, fb: &mut FrameBuffer) {
    cf(Hud {})
        .border(BorderStyle { foreground: Rgba32::new_grey(128), ..Default::default() })
        .render(scope, ctx, fb);
}
