use super::*;

impl World {
    pub fn character_walk_in_direction<R: Rng>(
        &mut self,
        _rng: &mut R,
        character: Entity,
        direction: CardinalDirection,
    ) -> Result<Option<crate::ControlFlow>, ActionError> {
        let current_coord = if let Some(coord) = self.spatial_table.coord_of(character) {
            coord
        } else {
            panic!("failed to find coord for {:?}", character);
        };

        let destination = current_coord + direction.coord();
        if let Some(&layers) = self.spatial_table.layers_at(destination) {
            if let Some(feature) = layers.feature {
                if self.components.solid.contains(feature) {
                    return ActionError::err_cant_walk_there();
                }
            }

            if layers.floor.is_some() {
                let _ = self.spatial_table.update_coord(character, destination);
            }
        } else {
            return ActionError::err_cant_walk_there();
        }

        Ok(None)
    }
}
