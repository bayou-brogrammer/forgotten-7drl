use super::*;

#[derive(Debug)]
pub struct FireWeapon {
    pub(crate) private: Private,
    pub(crate) slot: RangedWeaponSlot,
}

impl FireWeapon {
    pub fn slot(&self) -> RangedWeaponSlot {
        self.slot
    }

    pub fn commit(self, scope: &mut StateScope, direction: CardinalDirection) -> GameState {
        scope.0.world.character_fire_bullet(
            scope.0.player_entity,
            scope.0.player_coord() + (direction.coord() * 100),
            self.slot,
        );

        GameState::Running(Running(self.private))
    }

    pub fn cancel(self) -> GameState {
        GameState::Running(Running(self.private))
    }
}
