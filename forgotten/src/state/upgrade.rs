use super::*;

#[derive(Debug)]
pub struct UpgradeState(pub(crate) Private);

impl UpgradeState {
    pub fn commit(self, scope: &mut StateScope, upgrade: crate::Upgrade) -> GameState {
        let _ = scope.0.world.apply_upgrade(scope.0.player_entity, upgrade);
        GameState::Running(Running(self.0))
    }

    pub const fn cancel(self) -> GameState {
        GameState::Running(Running(self.0))
    }
}
