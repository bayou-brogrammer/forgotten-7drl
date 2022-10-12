use crate::TurnState;

use super::*;

#[derive(Debug)]
pub struct FireWeapon {
    pub(crate) private: Private,
    pub(crate) slot: RangedWeaponSlot,
}

impl FireWeapon {
    pub const fn slot(&self) -> RangedWeaponSlot {
        self.slot
    }

    pub fn commit(self, scope: &mut StateScope, direction: CardinalDirection) -> GameState {
        scope.0.world.character_fire_bullet(
            scope.0.player_entity,
            scope.0.player_coord() + (direction.coord() * 100),
            self.slot,
        );

        scope.0.turn_state = TurnState::EnemyTurn;
        GameState::Running(Running(self.private))
    }

    pub const fn cancel(self) -> GameState {
        GameState::Running(Running(self.private))
    }
}
