use crate::prelude::*;

// Spatial
impl World {
    pub fn entity_coord(&self, entity: Entity) -> Option<Coord> {
        self.spatial_table.coord_of(entity)
    }

    pub fn get_character_at_coord(&self, coord: Coord) -> Option<Entity> {
        self.spatial_table.layers_at(coord).and_then(|cell| cell.character)
    }

    pub fn can_npc_traverse_feature_at_coord(&self, coord: Coord) -> bool {
        if let Some(spatial_cell) = self.spatial_table.layers_at(coord) {
            if spatial_cell.floor.is_none() {
                return false;
            }
            if let Some(feature) = spatial_cell.feature {
                self.components.door_state.contains(feature)
                    || self.components.grass_state.contains(feature)
                    || !(self.components.solid.contains(feature))
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn is_npc_at_coord(&self, coord: Coord) -> bool {
        if let Some(spatial_cell) = self.spatial_table.layers_at(coord) {
            if let Some(entity) = spatial_cell.character {
                self.components.npc.contains(entity)
            } else {
                false
            }
        } else {
            false
        }
    }
}

// Visibility
impl World {
    pub fn can_npc_see_through_feature_at_coord(&self, coord: Coord) -> bool {
        if let Some(spatial_cell) = self.spatial_table.layers_at(coord) {
            if let Some(feature) = spatial_cell.feature {
                self.components.opacity.get(feature).cloned().unwrap_or(0) < 127
            } else {
                true
            }
        } else {
            false
        }
    }
}

// Entity
impl World {
    pub fn entity_npc(&self, entity: Entity) -> &Npc {
        self.components.npc.get(entity).unwrap()
    }

    pub fn entity_exists(&self, entity: Entity) -> bool {
        self.entity_allocator.exists(entity) && !self.components.dead.contains(entity)
    }
}
