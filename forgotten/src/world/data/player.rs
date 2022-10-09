use crate::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub credit: u32,
    pub melee_weapon: Weapon,
    pub ranged_weapons: [Option<Weapon>; 2],
}

impl Player {
    pub fn new() -> Self {
        Self { melee_weapon: Weapon::new_bare_hands(), credit: 0, ranged_weapons: [None, None] }
    }

    pub fn melee_dmg(&self) -> u32 {
        self.melee_weapon.dmg
    }

    pub fn melee_pen(&self) -> u32 {
        self.melee_weapon.pen
    }
}
