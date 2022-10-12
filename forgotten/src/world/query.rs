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
            spatial_cell.feature.map_or(true, |feature| {
                self.components.door_state.contains(feature)
                    || self.components.grass_state.contains(feature)
                    || !(self.components.solid.contains(feature))
            })
        } else {
            false
        }
    }

    #[allow(clippy::collapsible_match)]
    pub fn is_npc_at_coord(&self, coord: Coord) -> bool {
        if let Some(&Layers { character, .. }) = self.spatial_table.layers_at(coord) {
            character.map_or(false, |entity| self.components.npc.contains(entity))
        } else {
            false
        }
    }

    pub fn weapon_under_entity(&self, entity: Entity) -> Option<&Weapon> {
        self.spatial_table
            .layers_at(self.spatial_table.coord_of(entity)?)
            .and_then(|layers| layers.item.and_then(|item_entity| self.components.weapon.get(item_entity)))
    }
}

// Visibility
impl World {
    pub fn can_npc_see_through_feature_at_coord(&self, coord: Coord) -> bool {
        if let Some(Layers { feature, .. }) = self.spatial_table.layers_at(coord) {
            feature.map_or(true, |feature| self.components.opacity.get(feature).cloned().unwrap_or(0) < 127)
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

    pub fn entity_player(&self, entity: Entity) -> Option<&Player> {
        self.components.player.get(entity)
    }

    pub fn entity_exists(&self, entity: Entity) -> bool {
        self.entity_allocator.exists(entity) && !self.components.dead.contains(entity)
    }

    pub fn character_info(&self, entity: Entity) -> Option<CharacterInfo> {
        let coord = self.spatial_table.coord_of(entity)?;
        let &hit_points = self.components.hp.get(entity)?;
        let stunned = self.components.stunned.contains(entity);
        Some(CharacterInfo { coord, hit_points, stunned })
    }

    pub fn check_movement_blocked(&self, entity: Entity) -> bool {
        let is_blocked_mov = self.entity_coord(entity).map_or(false, |coord| {
            self.components.pushed_from.get(entity).map_or(false, |from| from.manhattan_distance(coord) > 1)
        });

        (is_blocked_mov && self.components.realtime.get(entity).is_some())
            || self.components.stunned.get(entity).is_some()
    }

    pub fn entity_has_ammo(&self, entity: Entity, slot: RangedWeaponSlot) -> bool {
        self.entity_player(entity).map_or_else(
            || {
                self.components.npc.get(entity).map_or(false, |npc| {
                    npc.weapon.as_ref().map_or(false, |weapon| {
                        weapon.ammo.unwrap_or(Ammo { current: 0, max: 0 }).current > 0
                    })
                })
            },
            |player| {
                player
                    .weapon_in_slot(slot)
                    .map_or(false, |weapon| weapon.ammo.unwrap_or(Ammo { current: 0, max: 0 }).current > 0)
            },
        )
    }
}

// Gameplay
impl World {
    pub fn is_gameplay_blocked(&self) -> bool {
        !self.components.blocks_gameplay.is_empty()
    }
}
