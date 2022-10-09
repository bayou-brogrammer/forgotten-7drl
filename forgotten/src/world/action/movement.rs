use super::*;

impl World {
    pub fn character_walk_in_direction(
        &mut self,
        character: Entity,
        direction: CardinalDirection,
    ) -> Result<Option<crate::ControlFlow>, ActionError> {
        // Prevent NPC from moving while being knocked back
        if self.components.realtime.get(character).is_some() {
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

    pub fn projectile_move(&mut self, projectile_entity: Entity, movement_direction: Direction) {
        if let Some(current_coord) = self.spatial_table.coord_of(projectile_entity) {
            let next_coord = current_coord + movement_direction.coord();
            let collides_with =
                self.components.collides_with.get(projectile_entity).cloned().unwrap_or_default();

            if let Some(&spatial_cell) = self.spatial_table.layers_at(next_coord) {
                // Is there a character here?
                if let Some(character_entity) = spatial_cell.character {
                    if let Some(&projectile_damage) = self.components.projectile_damage.get(projectile_entity)
                    {
                        self.apply_projectile_damage(
                            projectile_entity,
                            projectile_damage,
                            movement_direction,
                            character_entity,
                        );
                    }
                }

                // Is there a feature here? TODO: self.components.enemy || self.components.npc??
                if let Some(entity_in_cell) = spatial_cell.feature.or(spatial_cell.character) {
                    if (collides_with.solid && self.components.solid.contains(entity_in_cell))
                        || (collides_with.character && self.components.enemy.contains(entity_in_cell))
                    {
                        let mut stop = true;
                        if let Some(&projectile_damage) =
                            self.components.projectile_damage.get(projectile_entity)
                        {
                            if self.components.destructible.contains(entity_in_cell) {
                                let hull_pen_percent = projectile_damage.hull_pen_percent;
                                if crate::rng::range(0..=100) < hull_pen_percent {
                                    self.components.remove_entity(entity_in_cell);
                                    self.spatial_table.remove(entity_in_cell);
                                    stop = false;
                                }
                            }
                        }

                        // Slammed against a wall
                        if self.realtime_components.movement.contains(projectile_entity) {
                            let from_coord = self.components.pushed_from.get(projectile_entity).unwrap();
                            let distance = current_coord.manhattan_distance(*from_coord);
                            let dmg = if distance >= 2 { 2 } else { distance };
                            self.damage_character(projectile_entity, dmg)
                        }

                        if stop {
                            self.projectile_stop(projectile_entity);
                            return;
                        }
                    }
                }

                let _ignore_err = self.spatial_table.update_coord(projectile_entity, next_coord);
            } else {
                self.projectile_stop(projectile_entity);
            }
        } else {
            self.clear_entity(projectile_entity);
        }
    }

    pub fn projectile_stop(&mut self, projectile_entity: Entity) {
        if let Some(_current_coord) = self.spatial_table.coord_of(projectile_entity) {
            if let Some(on_collision) = self.components.on_collision.get(projectile_entity) {
                match on_collision {
                    OnCollision::Remove => {
                        self.spatial_table.remove(projectile_entity);
                        self.components.remove_entity(projectile_entity);
                        self.entity_allocator.free(projectile_entity);
                        self.realtime_components.remove_entity(projectile_entity);
                    }
                    OnCollision::RemoveRealtime => {
                        self.realtime_components.remove_entity(projectile_entity);
                        self.components.realtime.remove(projectile_entity);
                    }
                }
            }
        }

        // TODO: This might cause issues?
        self.components.realtime.remove(projectile_entity);
        self.realtime_components.movement.remove(projectile_entity);
    }
}
