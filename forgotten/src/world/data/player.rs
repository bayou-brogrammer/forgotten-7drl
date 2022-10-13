use crate::prelude::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub credit: u32,
    pub traits: PlayerTraits,
    pub melee_weapon: Weapon,
    pub upgrade_table: UpgradeTable,
    pub ranged_weapons: Vec<Option<Weapon>>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            credit: 0,
            traits: Default::default(),
            ranged_weapons: vec![None, None],
            melee_weapon: Weapon::new_bare_hands(),
            upgrade_table: UpgradeTable { toughness: None, accuracy: None, endurance: None },
        }
    }

    pub fn weapon_in_slot(&self, slot: RangedWeaponSlot) -> Option<&Weapon> {
        if slot.index() >= self.ranged_weapons.len() {
            return None;
        }

        self.ranged_weapons[slot.index()].as_ref()
    }

    pub fn stun_percent(&self) -> u8 {
        self.melee_weapon.stun_percent.unwrap_or(0)
    }

    pub const fn melee_dmg(&self) -> u32 {
        self.melee_weapon.dmg
    }

    pub const fn melee_pen(&self) -> u32 {
        self.melee_weapon.pen
    }

    pub fn available_upgrades(&self) -> Vec<Upgrade> {
        let mut out = Vec::new();
        match self.upgrade_table.toughness {
            None => out.push(Upgrade { typ: UpgradeType::Toughness, level: UpgradeLevel::Level1 }),
            Some(UpgradeLevel::Level1) => {
                out.push(Upgrade { typ: UpgradeType::Toughness, level: UpgradeLevel::Level2 })
            }
            Some(UpgradeLevel::Level2) => {
                out.push(Upgrade { typ: UpgradeType::Toughness, level: UpgradeLevel::Level3 })
            }
            Some(UpgradeLevel::Level3) => (),
        }

        out
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
    pub explosive_damage: bool,
}
