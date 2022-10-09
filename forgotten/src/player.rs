use crate::{prelude::*, TurnState};

pub enum Input {
    Wait,
    Walk(CardinalDirection),
}

impl Game {
    pub fn player_coord(&self) -> Coord {
        self.world.spatial_table.coord_of(self.player_entity).expect("can't find coord of player")
    }

    pub fn player_walk(&mut self, direction: CardinalDirection) -> Result<Option<ControlFlow>, ActionError> {
        let flow = self.world.character_walk_in_direction(self.player_entity, direction)?;
        self.turn_state = TurnState::EnemyTurn;
        Ok(flow)
    }

    pub fn player_wait(&mut self) -> Option<ControlFlow> {
        self.turn_state = TurnState::EnemyTurn;
        None
    }

    // pub fn player_input(&mut self, input: Input) -> Result<Option<ControlFlow>, ActionError> {
    //     let flow = match input {
    //         Input::Wait => Ok(None),
    //         Input::Walk(direction) => self.world.character_walk_in_direction(self.player_entity, direction),
    //     };

    //     self.update_visibility();
    //     self.turn_state = TurnState::EnemyTurn;

    //     flow
    // }
}
