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

impl Tile {
    pub fn is_wall(&self) -> bool {
        matches!(self, Self::Wall | Self::DoorClosed | Self::DoorOpen | Self::CaveWall)
    }
}

entity_table::declare_entity_module! {
    components {
        // Visibility
        opacity: u8,
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
        enemy: (),
        player: Player,

        // Stats
        dead: (),
        hp: HitPoints,
        armour: Armour,

        // Animation / Projectile
        animating: (),
        realtime: (),
        on_collision: OnCollision,
        collides_with: CollidesWith,
        projectile_damage: ProjectileDamage,
        pushed_from: Coord,

    }
}

pub use components::{Components, EntityData, EntityUpdate};
