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

        let stunned_txt = if player_info.stunned { "** Stunned **" } else { "" };
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
            plain(stunned_txt),
        ];
        Text::from(text).render(&(), ctx, fb);

        render_weapon("Melee:", &player.melee_weapon, player, ctx.add_y(5), fb);

        let ctx = ctx.add_y(13);
        for (i, ranged_slot) in player.ranged_weapons.iter().enumerate() {
            if let Some(weapon) = ranged_slot {
                render_weapon(
                    format!("Ranged {}:", i + 1).as_str(),
                    weapon,
                    player,
                    ctx.add_y(i as i32 * 7),
                    fb,
                );
            } else {
                render_empty_weapon_slot(format!("Ranged {}:", i + 1).as_str(), ctx.add_y(i as i32 * 10), fb);
            }
        }
    }

    fn update(&mut self, _: &mut Self::State, _: Ctx, _: Event) -> Self::Output {}

    fn size(&self, _: &Self::State, ctx: Ctx) -> Size {
        Size::new(ctx.bounding_box.size().width() + 1, GAME_VIEW_SIZE.height() - 1)
    }
}

pub fn render_hud(scope: &StateScope, ctx: Ctx, fb: &mut FrameBuffer) {
    cf(Hud {})
        .border(BorderStyle { foreground: Rgba32::new_grey(128), ..Default::default() })
        .render(scope, ctx, fb);
}
