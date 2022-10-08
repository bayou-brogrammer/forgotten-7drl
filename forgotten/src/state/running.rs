use super::*;

/// Wraps a `Game`, and can only be constructed from a `Running`, serving as proof that the wrapped
/// game is in the state represented by the `Running` witness
#[derive(Serialize, Deserialize)]
pub struct RunningGame(StateScope);

impl RunningGame {
    pub fn into_game(self) -> (StateScope, Running) {
        (self.0, Running(Private))
    }
}

/// Represents the fact that the game is currently running, expecting inputs that interact with the
/// game world by manipulating the player character
#[derive(Debug)]
pub struct Running(pub(super) Private);

impl Running {
    pub fn running_game(self, scope: StateScope) -> RunningGame {
        RunningGame(scope)
    }

    /// Convenience method for wrapping `self` in `GameState::Running(...)`
    pub fn into_witness(self) -> GameState {
        GameState::Running(self)
    }

    /// Helper for turning self into a prompt with a given message
    fn into_prompt_witness(self, message: String) -> GameState {
        GameState::Prompt(Prompt { message, private: self.0 })
    }

    fn handle_control_flow(self, cf: Option<ControlFlow>) -> GameState {
        match cf {
            None => self.into_witness(),
            Some(control_flow) => match control_flow {
                ControlFlow::Prompt(message) => self.into_prompt_witness(message),
                // ControlFlow::Sleep => GameState::Sleep(Sleep(self.0)),
                ControlFlow::Win => GameState::Win,
                ControlFlow::GameOver => GameState::GameOver,
                ControlFlow::LevelChange => todo!(),
            },
        }
    }

    /// Common logic for handling the common return type of methods that update the game state
    fn handle_control_flow_result(
        self,
        cfr: Result<Option<ControlFlow>, ActionError>,
    ) -> (GameState, Result<(), ActionError>) {
        match cfr {
            Ok(maybe_control_flow) => (self.handle_control_flow(maybe_control_flow), Ok(())),
            Err(e) => (self.into_witness(), Err(e)),
        }
    }

    /// Called periodically, once per frame
    pub fn tick(self, scope: &mut StateScope, since_previous: Duration) -> GameState {
        self.handle_control_flow(scope.0.tick(since_previous))
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
// Actions
//////////////////////////////////////////////////////////////////////////////////////////

impl Running {
    pub fn player_walk(
        self,
        game: &mut StateScope,
        direction: CardinalDirection,
    ) -> (GameState, Result<(), ActionError>) {
        self.handle_control_flow_result(game.0.player_walk(direction))
    }
}
