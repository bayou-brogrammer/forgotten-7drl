use crate::{prelude::*, prompt, TurnState};

pub enum ControlFlow {
    Win,
    GetMelee,
    GameOver,
    GetRanged,
    LevelChange,
    Prompt(String),
    FireWeapon(RangedWeaponSlot),
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
        if self.turn_state == TurnState::EnemyTurn {
            self.npc_turn();
            self.turn_state = TurnState::PlayerTurn;
        }

        self.world.run_systems(&mut self.agents, &mut self.animation_context);
        self.update_visibility();

        if self.is_game_over() {
            Some(ControlFlow::GameOver)
        } else if self.is_won() {
            Some(ControlFlow::Win)
        } else {
            None
        }
    }
}
