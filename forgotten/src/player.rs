use crate::{prelude::*, TurnState};

pub enum Input {
    Wait,
    EquipMeleeWeapon,
    Walk(CardinalDirection),
    EquipRangedWeapon(RangedWeaponSlot),
}

impl Game {
    pub fn player_coord(&self) -> Coord {
        self.world.spatial_table.coord_of(self.player_entity).expect("can't find coord of player")
    }

    pub fn player_has_third_weapon_slot(&self) -> bool {
        let player = self.world.components.player.get(self.player_entity).unwrap();
        player.ranged_weapons.len() == 3
    }

    pub fn player_has_melee_weapon_equipped(&self) -> bool {
        let player = self.world.components.player.get(self.player_entity).unwrap();
        player.melee_weapon.is_melee()
    }

    pub fn player_has_weapon_in_slot(&self, slot: RangedWeaponSlot) -> bool {
        let player = self.world.components.player.get(self.player_entity).unwrap();
        if slot.index() >= player.ranged_weapons.len() {
            return false;
        }
        player.ranged_weapons[slot.index()].is_some()
    }

    // Actions

    pub fn player_walk(&mut self, direction: CardinalDirection) -> Result<Option<ControlFlow>, ActionError> {
        let flow = self.world.character_walk_in_direction(self.player_entity, direction)?;
        self.turn_state = TurnState::EnemyTurn;
        Ok(flow)
    }

    pub fn player_wait(&mut self) -> Option<ControlFlow> {
        self.turn_state = TurnState::EnemyTurn;
        None
    }

    pub fn player_get(&mut self) -> Result<Option<ControlFlow>, ActionError> {
        if let Some(weapon) = self.world.weapon_under_entity(self.player_entity) {
            if weapon.is_ranged() {
                return Ok(Some(ControlFlow::GetRanged));
            }

            if weapon.is_melee() {
                return Ok(Some(ControlFlow::GetMelee));
            }
        }

        ActionError::no_item_there()
    }

    pub fn player_fire(&self, slot: RangedWeaponSlot) -> Result<Option<ControlFlow>, ActionError> {
        if let Some(player) = self.player() {
            if let Some(weapon) = player.weapon_in_slot(slot) {
                if weapon.ammo.unwrap().current == 0 {
                    return ActionError::out_of_ammo(weapon.name);
                } else {
                    return Ok(Some(ControlFlow::FireWeapon(slot)));
                }
            }
        }

        ActionError::no_weapon_in_slot(slot)
    }
}
