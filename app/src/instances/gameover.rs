use super::popup_delay;
use crate::prelude::*;
use gridbugs::chargrid::control_flow::on_state_then;

// struct GameOverComponent(Option<state::GameOver>);

// impl Component for GameOverComponent {
//     type Output = ();
//     type State = GameLoopData;

//     fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
//         StyledString::plain_text("Game over!".to_string()).render(&(), ctx, fb);
//     }

//     fn update(&mut self, state: &mut Self::State, _ctx: Ctx, event: Event) -> Self::Output {
//         // let game_over = self.0.take().unwrap();
//         // self.0 = Some(state.game_over_tick(event, game_over));
//     }

//     fn size(&self, _state: &Self::State, ctx: Ctx) -> Size {
//         ctx.bounding_box.size()
//     }
// }

pub fn game_over() -> AppCF<()> {
    on_state_then(move |state: &mut State| {
        state.examine_message = None;
        state.cursor = None;
        state.clear_saved_game();
        popup_delay("You tire of trudging through the flooded forest in the rain. You pack up your belongings and return home.".to_string())
    })
}
