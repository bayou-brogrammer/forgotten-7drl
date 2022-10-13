use super::GetRangedWeapon;
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

    fn into_ranged_witness(self) -> GameState {
        GameState::GetRangedWeapon(GetRangedWeapon(self.0))
    }

    fn into_melee_witness(self) -> GameState {
        GameState::GetMeleeWeapon(GetMeleeWeapon(self.0))
    }

    fn into_fire_witness(self, slot: RangedWeaponSlot) -> GameState {
        GameState::FireWeapon(FireWeapon { slot, private: self.0 })
    }

    fn into_upgrade(self) -> GameState {
        GameState::Upgrade(UpgradeState(self.0))
    }

    pub(crate) fn handle_control_flow(self, cf: Option<ControlFlow>) -> GameState {
        match cf {
            None => self.into_witness(),
            Some(control_flow) => match control_flow {
                ControlFlow::Win => GameState::Win,
                ControlFlow::LevelChange => self.into_witness(),
                ControlFlow::GameOver => GameState::GameOver,
                ControlFlow::GetMelee => self.into_melee_witness(),
                ControlFlow::GetRanged => self.into_ranged_witness(),
                ControlFlow::Prompt(message) => self.into_prompt_witness(message),
                ControlFlow::FireWeapon(slot) => self.into_fire_witness(slot),
                ControlFlow::Upgrade => self.into_upgrade(),
            },
        }
    }

    /// Common logic for handling the common return type of methods that update the game state
    pub(crate) fn handle_control_flow_result(
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

    pub fn player_wait(self, game: &mut StateScope) -> GameState {
        self.handle_control_flow(game.0.player_wait())
    }

    pub fn player_get(self, game: &mut StateScope) -> (GameState, Result<(), ActionError>) {
        self.handle_control_flow_result(game.0.player_get())
    }

    pub fn player_fire_weapon(
        self,
        game: &StateScope,
        slot: RangedWeaponSlot,
    ) -> (GameState, Result<(), ActionError>) {
        self.handle_control_flow_result(game.0.player_fire(slot))
    }

    pub fn player_descend(self, game: &mut StateScope) -> (GameState, Result<(), ActionError>) {
        self.handle_control_flow_result(game.0.player_descend())
    }
}
