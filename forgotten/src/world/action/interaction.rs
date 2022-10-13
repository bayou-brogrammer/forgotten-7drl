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

    pub fn equip_melee_weapon_from_ground(&mut self, entity: Entity) {
        if let Some(coord) = self.spatial_table.coord_of(entity) {
            if let Some((item_entity, weapon)) = self.spatial_table.layers_at(coord).and_then(|layers| {
                layers.item.and_then(|item_entity| {
                    self.components.weapon.get(item_entity).map(|weapon| (item_entity, weapon.clone()))
                })
            }) {
                if weapon.is_melee() {
                    if let Some(player) = self.components.player.get_mut(entity) {
                        crate::log::append_entry(Message::EquipWeapon(weapon.name));
                        player.melee_weapon = weapon;
                        self.components.dead.insert(item_entity, ());
                    }
                }
            }
        }
    }

    pub fn equip_ranged_weapon_from_ground(&mut self, entity: Entity, slot: RangedWeaponSlot) {
        if let Some(coord) = self.spatial_table.coord_of(entity) {
            if let Some((item_entity, weapon)) = self.spatial_table.layers_at(coord).and_then(|layers| {
                layers.item.and_then(|item_entity| {
                    self.components.weapon.get(item_entity).map(|weapon| (item_entity, weapon.clone()))
                })
            }) {
                if weapon.is_ranged() {
                    if let Some(player) = self.components.player.get_mut(entity) {
                        crate::log::append_entry(Message::EquipWeapon(weapon.name));
                        player.ranged_weapons[slot.index()] = Some(weapon);
                        self.components.dead.insert(item_entity, ());
                    }
                }
            }
        }
    }

    pub fn apply_upgrade(&mut self, entity: Entity, upgrade: Upgrade) -> Result<(), ActionError> {
        let player = self.components.player.get_mut(entity).unwrap();
        if player.credit < upgrade.level.cost() {
            return ActionError::cannot_afford_upgrade();
        }

        player.credit -= upgrade.level.cost();
        {
            let player_level = match upgrade.typ {
                UpgradeType::Toughness => &mut player.upgrade_table.toughness,
            };

            *player_level = Some(upgrade.level);
        }

        use UpgradeLevel::*;
        use UpgradeType::*;
        match upgrade {
            Upgrade { typ: Toughness, level: Level1 } => {
                player.ranged_weapons.push(None);
            }
            Upgrade { typ: Toughness, level: Level2 } => {
                let hit_points = self.components.hp.get_mut(entity).unwrap();
                hit_points.max *= 2;
                hit_points.current *= 2;
            }
            Upgrade { typ: Toughness, level: Level3 } => {
                player.traits.explosive_damage = true;
            }
        }

        Ok(())
    }
}
