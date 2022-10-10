use super::*;

impl World {
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

                // Is there a feature here?
                if let Some(entity_in_cell) = spatial_cell.feature.or(spatial_cell.character) {
                    if (collides_with.solid && self.components.solid.contains(entity_in_cell))
                        || (collides_with.character && self.components.character.contains(entity_in_cell))
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
                        if self.realtime_components.movement.contains(projectile_entity)
                            && self.components.character.contains(projectile_entity)
                        {
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
