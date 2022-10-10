use crate::prelude::*;

impl World {
    pub fn character_walk_in_direction(
        &mut self,
        character: Entity,
        direction: CardinalDirection,
    ) -> Result<Option<crate::ControlFlow>, ActionError> {
        // Prevent NPC from moving while being knocked back
        if self.check_movement_blocked(character) {
            self.reduce_stun(character);
            return Ok(None);
        }

        let spatial_table = &mut self.spatial_table;
        let current_coord = if let Some(coord) = spatial_table.coord_of(character) {
            coord
        } else {
            panic!("failed to find coord for {:?}", character);
        };

        let target_coord = current_coord + direction.coord();
        if let Some(&Layers { feature: Some(feature_entity), .. }) =
            self.spatial_table.layers_at(target_coord)
        {
            // If the player bumps into a door, open the door
            if let Some(DoorState::Closed) = self.components.door_state.get(feature_entity) {
                self.open_door(feature_entity);
                return Ok(None);
            }

            // Don't let the player walk through solid entities
            if self.components.solid.contains(feature_entity) {
                if let Some(open_door_entity) = self.open_door_entity_adjacent_to_coord(target_coord) {
                    self.close_door(open_door_entity);
                    return Ok(None);
                }
                return ActionError::err_cant_walk_there();
            }

            if let Some(GrassState::Normal) = self.components.grass_state.get(feature_entity) {
                self.crush_grass(feature_entity);
            }
        }

        if let Err(occupant) =
            self.spatial_table.update_coord(character, target_coord).map_err(|e| e.unwrap_occupied_by())
        {
            self.melee_attack(character, occupant, direction);
        }

        Ok(None)
    }

    pub fn character_fire_bullet(&mut self, character: Entity, target: Coord, slot: RangedWeaponSlot) {
        let character_coord = self.spatial_table.coord_of(character).unwrap();
        if character_coord == target {
            return;
        }

        let player = self.components.player.get_mut(character).unwrap();
        if let Some(weapon) = player.ranged_weapons[slot.index()].as_mut() {
            if let Some(ammo) = weapon.ammo.as_mut() {
                if ammo.current == 0 {
                    return;
                } else {
                    ammo.current -= 1;
                }
            }

            let weapon = weapon.clone();
            let sound_effect = match weapon.name {
                WeaponType::BareHands | WeaponType::CattleProd | WeaponType::Chainsaw => None,
                WeaponType::Railgun => Some(SoundEffect::Railgun),
                WeaponType::LifeStealer => Some(SoundEffect::LifeStealer),
                WeaponType::FiftyCal => todo!(),
            };

            if let Some(sound_effect) = sound_effect {
                crate::event::add_event(ExternalEvent::SoundEffect(sound_effect));
            }

            self.spawn_bullet(character_coord, target, &weapon);
            self.spawn_flash(character_coord, None);
        }
    }
}
