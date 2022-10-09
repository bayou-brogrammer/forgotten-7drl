use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ammo {
    pub current: u32,
    pub max: u32,
}

impl Ammo {
    pub fn new_full(max: u32) -> Self {
        Self { current: max, max }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponType {
    // Melee
    #[default]
    BareHands,
    CattleProd,
    Chainsaw,

    // Ranged
    Railgun,
    LifeStealer,
}

impl WeaponType {
    pub fn tile(self) -> Tile {
        use WeaponType::*;
        match self {
            CattleProd => Tile::Weapon(CattleProd),
            Chainsaw => Tile::Weapon(Chainsaw),
            Railgun => Tile::Weapon(Railgun),
            LifeStealer => Tile::Weapon(LifeStealer),
            BareHands => Tile::Weapon(BareHands),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Weapon {
    pub pen: u32,
    pub dmg: u32,
    pub bright: bool,
    pub name: WeaponType,
    pub ammo: Option<Ammo>,
    pub hull_pen_percent: u32,
    pub light_colour: Option<Rgb24>,
    pub abilities: Vec<WeaponAbility>,
}

impl Weapon {
    pub fn new_bare_hands() -> Self {
        Self {
            pen: 0,
            dmg: 1,
            ammo: None,
            bright: false,
            light_colour: None,
            hull_pen_percent: 0,
            name: WeaponType::BareHands,
            abilities: vec![WeaponAbility::KnockBack],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponAbility {
    KnockBack,
}
