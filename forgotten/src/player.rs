use crate::{prelude::*, TurnState};

impl Game {
    pub fn player_coord(&self) -> Coord {
        self.world.spatial_table.coord_of(self.player_entity).expect("can't find coord of player")
    }

    pub fn player_walk(&mut self, direction: CardinalDirection) -> Result<Option<ControlFlow>, ActionError> {
        let flow = self.world.character_walk_in_direction(self.player_entity, direction)?;
        self.update_visibility();
        self.turn_state = TurnState::EnemyTurn;
        Ok(flow)
    }
}
