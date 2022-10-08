use crate::{prelude::*, prompt, TurnState};

pub enum ControlFlow {
    Win,
    GameOver,
    LevelChange,
    Prompt(String),
}

impl Game {
    #[must_use]
    pub fn tick(&mut self, since_previous: Duration) -> Option<ControlFlow> {
        if self.start {
            self.start = false;
            return Some(ControlFlow::Prompt(prompt::intro()));
        }

        self.since_last_frame += since_previous;
        while let Some(remaining_since_last_frame) =
            self.since_last_frame.checked_sub(crate::world::ANIMATION_FRAME_DURATION)
        {
            self.since_last_frame = remaining_since_last_frame;
            if let Some(game_control_flow) = self.handle_tick_inner(since_previous) {
                return Some(game_control_flow);
            }
        }

        None
    }

    pub fn handle_tick_inner(&mut self, since_previous: Duration) -> Option<ControlFlow> {
        // self.animation_context.animation_tick(&mut self.animation_context, &mut self.animation_rng);
        self.update_visibility();

        if self.turn_state == TurnState::EnemyTurn {
            self.npc_turn();
            self.turn_state = TurnState::PlayerTurn;
        }

        if self.is_game_over() {
            Some(ControlFlow::GameOver)
        } else if self.is_won() {
            Some(ControlFlow::Win)
        } else {
            None
        }
    }
}
