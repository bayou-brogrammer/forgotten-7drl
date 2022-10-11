use super::*;
use crate::controls::AppInput;
use gridbugs::chargrid::text::StyledString;

//////////////////////////////////////////////////////////////////////////////////////////////
/// Examione Component
//////////////////////////////////////////////////////////////////////////////////////////////

struct GameExamineComponent;

impl Component for GameExamineComponent {
    type Output = Option<()>;
    type State = GameLoopData;

    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        state.render(CURSOR.with_a(128), ctx, fb);
    }

    fn update(&mut self, state: &mut Self::State, _ctx: Ctx, event: Event) -> Self::Output {
        if let Some(input) = event.input() {
            if let Some(direction) = state.controls.get_direction(input) {
                let cursor = state.cursor.unwrap_or_else(|| state.scope().player_coord());
                state.cursor = Some(cursor + direction.coord());
            }

            if let Some(AppInput::Examine) = state.controls.get(input) {
                return Some(());
            }
        }
        state.examine_mouse(event);
        state.update_examine_text();
        None
    }

    fn size(&self, _state: &Self::State, ctx: Ctx) -> Size {
        ctx.bounding_box.size()
    }
}

pub fn game_examine_component() -> AppCF<()> {
    on_state_then(|state: &mut State| {
        state.context_message = Some(StyledString {
            string: "Examining (escape to return to game)".to_string(),
            style: Style::plain_text(),
        });

        let cursor = state.cursor.unwrap_or_else(|| state.scope().player_coord());
        state.cursor = Some(cursor);

        cf(GameExamineComponent).catch_escape_or_start().map_val(|| ()).side_effect(|state: &mut State| {
            state.context_message = None;
            state.cursor = None;
        })
    })
}

//////////////////////////////////////////////////////////////////////////////////////////////
/// Examione With Mouse Component
//////////////////////////////////////////////////////////////////////////////////////////////

pub struct GameExamineWithMouseComponent;

impl Component for GameExamineWithMouseComponent {
    type Output = ();
    type State = GameLoopData;

    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        state.render(CURSOR, ctx, fb);
    }

    fn update(&mut self, state: &mut Self::State, _ctx: Ctx, event: Event) -> Self::Output {
        state.examine_mouse(event);
    }

    fn size(&self, _state: &Self::State, ctx: Ctx) -> Size {
        ctx.bounding_box.size()
    }
}
