use crate::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub credit: u32,
    pub melee_weapon: Weapon,
    pub traits: PlayerTraits,
    pub ranged_weapons: [Option<Weapon>; 2],
}

impl Player {
    pub fn new() -> Self {
        Self {
            credit: 0,
            ranged_weapons: [None, None],
            traits: Default::default(),
            // melee_weapon: Weapon::new_cattle_prod(),
            melee_weapon: Weapon::new_bare_hands(),
        }
    }

    pub fn weapon_in_slot(&self, slot: RangedWeaponSlot) -> Option<&Weapon> {
        self.ranged_weapons[slot.index()].as_ref()
    }

    pub fn stun_percent(&self) -> u8 {
        self.melee_weapon.stun_percent.unwrap_or(0)
    }

    pub fn melee_dmg(&self) -> u32 {
        self.melee_weapon.dmg
    }

    pub fn melee_pen(&self) -> u32 {
        self.melee_weapon.pen
    }
}

#[derive(Serialize, Deserialize)]
pub struct CharacterInfo {
    pub coord: Coord,
    pub stunned: bool,
    pub hit_points: HitPoints,
}

////////////////////////
/// Traits
////////////////////////

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerTraits {
    pub double_stun: u8,
    pub double_damage: bool,
}
