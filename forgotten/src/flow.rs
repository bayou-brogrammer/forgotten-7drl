use crate::prelude::*;

pub enum ControlFlow {
    Win,
    GameOver,
    LevelChange,
    Prompt(String),
}

impl Game {
    #[must_use]
    pub fn tick(&mut self, since_previous: Duration, config: &GameConfig) -> Option<ControlFlow> {
        self.since_last_frame += since_previous;
        while let Some(remaining_since_last_frame) =
            self.since_last_frame.checked_sub(crate::world::ANIMATION_FRAME_DURATION)
        {
            self.since_last_frame = remaining_since_last_frame;
            if let Some(game_control_flow) = self.handle_tick_inner(since_previous, config) {
                return Some(game_control_flow);
            }
        }

        None
    }

    pub fn handle_tick_inner(
        &mut self,
        since_previous: Duration,
        config: &GameConfig,
    ) -> Option<ControlFlow> {
        // self.animation_context.animation_tick(&mut self.animation_context, &mut self.animation_rng);
        self.update_visibility(config);

        // if self.is_game_over() {
        //     Some(ControlFlow::GameOver)
        // } else if self.won {
        //     Some(ControlFlow::Win)
        // } else {
        //     None
        // }
        None
    }
}
