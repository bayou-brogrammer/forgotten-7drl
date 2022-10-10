use crate::prelude::*;
use gridbugs::{entity_table, visible_area_detection::*};

mod npc;
mod player;
mod projectile;
mod stats;
mod terrain;
mod weapon;

pub use npc::*;
pub use player::*;
pub use projectile::*;
pub use stats::*;
pub use terrain::*;
pub use weapon::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Tile {
    Player,
    Npc(NpcType),
    Weapon(WeaponType),
    Laser,
    Bullet,

    Wall,
    DoorOpen,
    DoorClosed,
    Floor,
    CaveWall,
    CaveFloor,
    Grass,
    GrassCrushed,
    Water,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Item {
    Medkit,
    Credit(u32),
    Weapon(WeaponType),
}

impl Tile {
    pub fn is_wall(&self) -> bool {
        matches!(self, Self::Wall | Self::DoorClosed | Self::DoorOpen | Self::CaveWall)
    }
}

entity_table::declare_entity_module! {
    components {
        // Visibility
        opacity: u8,
        colour_hint: Rgb24,
        vision: vision_distance::Circle,
        light: Light<vision_distance::Circle>,

        // Terrain
        solid: (),
        tile: Tile,
        destructible: (),
        grass_state: GrassState,
        door_state: DoorState,

        //Entity
        npc: Npc,
        item: Item,
        player: Player,
        stunned: Stunned,
        character: (),
        weapon: Weapon,

        // Stats
        dead: (),
        damage: u32,
        hp: HitPoints,
        armour: Armour,

        // Animation / Projectile
        particle: (),
        realtime: (),
        animating: (),
        pushed_from: Coord,
        on_collision: OnCollision,
        collides_with: CollidesWith,
        projectile_damage: ProjectileDamage,

    }
}

pub use components::{Components, EntityData, EntityUpdate};
