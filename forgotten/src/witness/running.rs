use super::*;

/// Represents the fact that the game is currently running, expecting inputs that interact with the
/// game world by manipulating the player character
#[derive(Debug)]
pub struct Running(pub(super) Private);

impl Running {
    pub fn running_game(self, scope: WitnessScope) -> RunningGame {
        RunningGame(scope)
    }

    /// Convenience method for wrapping `self` in `Witness::Running(...)`
    pub fn into_witness(self) -> Witness {
        Witness::Running(self)
    }

    /// Helper for turning self into a prompt with a given message
    fn into_prompt_witness(self, message: String) -> Witness {
        Witness::Prompt(Prompt { message, private: self.0 })
    }

    fn handle_control_flow(self, cf: Option<ControlFlow>) -> Witness {
        match cf {
            None => self.into_witness(),
            Some(control_flow) => match control_flow {
                ControlFlow::Prompt(message) => self.into_prompt_witness(message),
                // ControlFlow::Sleep => Witness::Sleep(Sleep(self.0)),
                ControlFlow::Win => Witness::Win,
                ControlFlow::GameOver => Witness::GameOver,
                ControlFlow::LevelChange => todo!(),
            },
        }
    }

    /// Common logic for handling the common return type of methods that update the game state
    fn handle_control_flow_result(
        self,
        cfr: Result<Option<ControlFlow>, ActionError>,
    ) -> (Witness, Result<(), ActionError>) {
        match cfr {
            Ok(maybe_control_flow) => (self.handle_control_flow(maybe_control_flow), Ok(())),
            Err(e) => (self.into_witness(), Err(e)),
        }
    }

    /// Called periodically, once per frame
    pub fn tick(self, scope: &mut WitnessScope, since_previous: Duration, config: &GameConfig) -> Witness {
        self.handle_control_flow(scope.0.tick(since_previous, config))
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
// Actions
//////////////////////////////////////////////////////////////////////////////////////////

impl Running {
    pub fn player_walk(
        self,
        game: &mut WitnessScope,
        direction: CardinalDirection,
        config: &GameConfig,
    ) -> (Witness, Result<(), ActionError>) {
        self.handle_control_flow_result(game.0.player_walk(direction, config))
    }
}
