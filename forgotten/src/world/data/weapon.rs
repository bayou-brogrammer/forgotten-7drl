use crate::{prelude::*, world::explosion};

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
    Pistol,
    Rifle,
    Railgun,
    Leecher,
    FiftyCal,
}

impl WeaponType {
    pub fn tile(self) -> Tile {
        use WeaponType::*;
        match self {
            Pistol => Tile::Weapon(Pistol),
            Rifle => Tile::Weapon(Rifle),
            CattleProd => Tile::Weapon(CattleProd),
            Chainsaw => Tile::Weapon(Chainsaw),
            Railgun => Tile::Weapon(Railgun),
            Leecher => Tile::Weapon(Leecher),
            BareHands => Tile::Weapon(BareHands),
            FiftyCal => Tile::Weapon(FiftyCal),
        }
    }

    pub fn new_weapon(self) -> Weapon {
        match self {
            WeaponType::FiftyCal => Weapon::new_fiftycal(),
            WeaponType::Railgun => Weapon::new_railgun(),
            WeaponType::Chainsaw => Weapon::new_chainsaw(),
            WeaponType::BareHands => Weapon::new_bare_hands(),
            WeaponType::CattleProd => Weapon::new_cattle_prod(),
            WeaponType::Leecher => Weapon::new_leecher(),
            WeaponType::Pistol => Weapon::new_pistol(),
            WeaponType::Rifle => Weapon::new_rifle(),
        }
    }
}

impl ToString for WeaponType {
    fn to_string(&self) -> String {
        match self {
            WeaponType::BareHands => "Bare Hands".to_string(),
            WeaponType::CattleProd => "Cattle Prod".to_string(),
            WeaponType::Chainsaw => "Chainsaw".to_string(),
            WeaponType::Railgun => "Railgun".to_string(),
            WeaponType::Leecher => "Leecher".to_string(),
            WeaponType::FiftyCal => "Fifty Cal".to_string(),
            WeaponType::Pistol => "Pistol".to_string(),
            WeaponType::Rifle => "Rifle".to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub pen: u32,
    pub dmg: u32,
    pub bright: bool,
    pub name: WeaponType,
    pub ammo: Option<Ammo>,
    pub hull_pen_percent: u32,
    pub stun_percent: Option<u8>,
    pub light_colour: Option<Rgb24>,
    pub abilities: Vec<WeaponAbility>,
    pub on_collision: Option<OnCollision>,
    pub collides_with: Option<CollidesWith>,
}

impl Weapon {
    pub fn is_ranged(&self) -> bool {
        matches!(
            self.name,
            WeaponType::Railgun
                | WeaponType::FiftyCal
                | WeaponType::Leecher
                | WeaponType::Pistol
                | WeaponType::Rifle
        )
    }

    pub fn is_melee(&self) -> bool {
        matches!(self.name, WeaponType::BareHands | WeaponType::CattleProd | WeaponType::Chainsaw)
    }

    pub fn new_bare_hands() -> Self {
        Self {
            pen: 1,
            dmg: 1,
            ammo: None,
            bright: false,
            on_collision: None,
            light_colour: None,
            stun_percent: None,
            hull_pen_percent: 0,
            collides_with: None,
            name: WeaponType::BareHands,
            abilities: vec![WeaponAbility::KnockBack],
        }
    }

    pub fn new_chainsaw() -> Self {
        Self {
            dmg: 5,
            pen: 10,
            bright: false,
            abilities: vec![],
            light_colour: None,
            stun_percent: None,
            on_collision: None,
            collides_with: None,
            hull_pen_percent: 0,
            name: WeaponType::Chainsaw,
            ammo: Some(Ammo::new_full(6)),
        }
    }

    pub fn new_cattle_prod() -> Self {
        Self {
            pen: 5,
            dmg: 3,
            bright: false,
            on_collision: None,
            light_colour: None,
            hull_pen_percent: 0,
            collides_with: None,
            stun_percent: Some(30),
            name: WeaponType::CattleProd,
            ammo: Some(Ammo::new_full(10)),
            abilities: vec![WeaponAbility::Shock],
        }
    }

    // Ranged

    pub fn new_leecher() -> Self {
        Self {
            dmg: 4,
            pen: 3,
            bright: false,
            stun_percent: None,
            hull_pen_percent: 0,
            name: WeaponType::Leecher,
            ammo: Some(Ammo::new_full(10)),
            on_collision: Some(OnCollision::Remove),
            light_colour: Some(Rgb24::new(75, 255, 0)),
            abilities: vec![WeaponAbility::LifeSteal],
            collides_with: Some(CollidesWith::default()),
        }
    }

    pub fn new_pistol() -> Self {
        Self {
            dmg: 2,
            pen: 3,
            bright: false,
            abilities: vec![],
            light_colour: None,
            hull_pen_percent: 40,
            stun_percent: Some(10),
            name: WeaponType::Pistol,
            ammo: Some(Ammo::new_full(10)),
            on_collision: Some(OnCollision::Remove),
            collides_with: Some(CollidesWith::default()),
        }
    }

    pub fn new_rifle() -> Self {
        Self {
            dmg: 4,
            pen: 6,
            bright: false,
            abilities: vec![],
            light_colour: None,
            hull_pen_percent: 40,
            stun_percent: Some(25),
            name: WeaponType::Rifle,
            ammo: Some(Ammo::new_full(4)),
            on_collision: Some(OnCollision::Remove),
            collides_with: Some(CollidesWith::default()),
        }
    }

    pub fn new_railgun() -> Self {
        Self {
            dmg: 10,
            pen: 100,
            bright: true,
            abilities: vec![],
            stun_percent: None,
            hull_pen_percent: 75,
            name: WeaponType::Railgun,
            ammo: Some(Ammo::new_full(4)),
            on_collision: Some(OnCollision::Remove),
            light_colour: Some(Rgb24::new(0, 255, 255)),
            collides_with: Some(CollidesWith::default()),
        }
    }

    pub fn new_fiftycal() -> Self {
        Self {
            dmg: 50,
            pen: 100,
            bright: true,
            abilities: vec![],
            hull_pen_percent: 100,
            stun_percent: Some(100),
            name: WeaponType::FiftyCal,
            ammo: Some(Ammo::new_full(2)),
            light_colour: Some(Rgb24::new(127, 0, 255)),
            collides_with: Some(CollidesWith { solid: true, character: true }),
            on_collision: Some(OnCollision::Explode(explosion::spec::Explosion {
                mechanics: explosion::spec::Mechanics(10),
                particle_emitter: explosion::spec::ParticleEmitter {
                    num_particles_per_frame: 50,
                    min_step: Duration::from_millis(10),
                    max_step: Duration::from_millis(30),
                    duration: Duration::from_millis(250),
                    fade_duration: Duration::from_millis(250),
                },
            })),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponAbility {
    KnockBack,
    Shock,
    LifeSteal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RangedWeaponSlot {
    Slot1,
    Slot2,
    Slot3,
}

impl RangedWeaponSlot {
    pub fn number(self) -> u32 {
        match self {
            Self::Slot1 => 1,
            Self::Slot2 => 2,
            Self::Slot3 => 3,
        }
    }
    pub fn index(self) -> usize {
        match self {
            Self::Slot1 => 0,
            Self::Slot2 => 1,
            Self::Slot3 => 2,
        }
    }
}
