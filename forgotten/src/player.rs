use crate::prelude::*;

impl Game {
    pub fn player_coord(&self) -> Coord {
        self.world.spatial_table.coord_of(self.player_entity).expect("can't find coord of player")
    }
}
