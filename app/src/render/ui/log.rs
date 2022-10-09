use crate::prelude::*;
use gridbugs::chargrid::{border::BorderStyle, text::Text};

const N: usize = 7;

pub struct Log {}

impl Component for Log {
    type Output = ();
    type State = StateScope;

    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        let messages = state.message_log();
        let start = messages.len().saturating_sub(N);
        for (i, message) in messages[start..].iter().enumerate() {
            let text = match message {
                Message::PlayerHitEnemy { enemy, weapon } => {
                    vec![
                        plain("You hit the "),
                        enemy_text(*enemy),
                        plain(" with your "),
                        weapon_name_text(*weapon),
                        plain("."),
                    ]
                }
                Message::EnemyHitPlayer(enemy) => {
                    vec![plain("The "), enemy_text(*enemy), plain(" hits you!")]
                }
                Message::Intro => vec![b("Have I been forgotten?")],
            };

            Text::from(text).render(&(), ctx.add_y(i as i32), fb);
        }
    }

    fn update(&mut self, _: &mut Self::State, _: Ctx, _: Event) -> Self::Output {}

    fn size(&self, _: &Self::State, ctx: Ctx) -> Size {
        Size::new(ctx.bounding_box.size().width() + 1, 8)
    }
}

pub fn render_message_log(scope: &StateScope, ctx: Ctx, fb: &mut FrameBuffer) {
    cf(Log {})
        .border(BorderStyle { foreground: Rgba32::new_grey(128), ..Default::default() })
        .render(scope, ctx, fb);
}
