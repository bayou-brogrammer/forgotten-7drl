use super::*;

impl World {
    pub fn open_door(&mut self, entity: Entity) {
        self.components.apply_entity_update(
            entity,
            entity_update!(
                solid: None,
                door_state: Some(DoorState::Open),
                tile: Some(Tile::DoorOpen),
                opacity: Some(0),
            ),
        );
    }

    pub fn close_door(&mut self, entity: Entity) {
        self.components.apply_entity_update(
            entity,
            entity_update!(
                solid: Some(()),
                door_state: Some(DoorState::Closed),
                tile: Some(Tile::DoorClosed),
                opacity: Some(255),
            ),
        );
    }

    pub fn open_door_entity_adjacent_to_coord(&self, coord: Coord) -> Option<Entity> {
        for direction in Direction::all() {
            let potential_door_coord = coord + direction.coord();
            if let Some(&Layers { feature: Some(feature_entity), .. }) =
                self.spatial_table.layers_at(potential_door_coord)
            {
                if let Some(DoorState::Open) = self.components.door_state.get(feature_entity) {
                    return Some(feature_entity);
                }
            }
        }
        None
    }

    pub fn crush_grass(&mut self, entity: Entity) {
        self.components.insert_entity_data(
            entity,
            entity_data! {
                grass_state: GrassState::Crushed,
                tile: Tile::GrassCrushed,
                opacity: 0,
            },
        );
    }
}
