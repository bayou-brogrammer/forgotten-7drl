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
                Message::Intro => vec![b("Have I been forgotten?")],

                // AI
                Message::EnemyHitPlayer(enemy) => {
                    vec![plain("The "), enemy_text(*enemy), plain(" hits you!")]
                }
                Message::EnemyDies(enemy) => {
                    vec![plain("The "), enemy_text(*enemy), plain(" dies.")]
                }
                Message::EnemyStunend(npc_type) => {
                    vec![plain("The "), enemy_text(*npc_type), plain(" is stunned.")]
                }

                // Player
                Message::PlayerHitEnemy { enemy, weapon } => {
                    vec![
                        plain("You hit the "),
                        enemy_text(*enemy),
                        plain(" with your "),
                        weapon_name_text(*weapon),
                        plain("."),
                    ]
                }
                Message::PlayerDies => vec![t("You die!", BOLD.with_foreground(Rgba32::new_rgb(255, 0, 0)))],
                Message::PlayerStunned => vec![plain("You have been stunned!")],
                Message::EquipWeapon(weapon) => {
                    vec![plain("You equip the "), weapon_name_text(*weapon), plain(".")]
                }
            };

            Text::new(text).wrap_word().render(&(), ctx.add_y(i as i32), fb);
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
        .render(scope, ctx, fb)
}
