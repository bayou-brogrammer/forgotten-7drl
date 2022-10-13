use crate::prelude::*;
use gridbugs::{entity_table, visible_area_detection::*};

mod npc;
mod player;
mod projectile;
mod stats;
mod terrain;
mod upgrade;
mod weapon;

pub use npc::*;
pub use player::*;
pub use projectile::*;
pub use stats::*;
pub use terrain::*;
pub use upgrade::*;
pub use weapon::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Tile {
    Wall,
    DoorOpen,
    DoorClosed,
    Floor,
    CaveWall,
    CaveFloor,
    Grass,
    GrassCrushed,
    Water,
    Reactor,
    Stairs,

    // Entity
    Player,
    Bullet,
    Npc(NpcType),

    // Items
    Weapon(WeaponType),
    Medkit,
    Upgrade,
    Credit1,
    Credit2,
    Credit3,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Item {
    Medkit,
    Credit(u32),
    Weapon(WeaponType),
}

impl Tile {
    pub const fn is_wall(&self) -> bool {
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
        grass_state: GrassState,
        door_state: DoorState,
        stairs: (),
        upgrade: (),

        //Entity
        npc: Npc,
        item: Item,
        player: Player,
        stunned: Stunned,
        character: (),
        weapon: Weapon,
        reactor: (),

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
        explodes_on_death: (),
        blocks_gameplay: Duration,
        on_collision: OnCollision,
        collides_with: CollidesWith,
        projectile_damage: ProjectileDamage,

    }
}

pub use components::{Components, EntityData, EntityUpdate};
