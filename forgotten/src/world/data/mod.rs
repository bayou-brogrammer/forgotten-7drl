use crate::prelude::*;
use gridbugs::{entity_table, visible_area_detection::*};

mod description;
mod npc;
mod terrain;

pub use description::*;
pub use npc::*;
pub use terrain::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Tile {
    RoomWall,
    DoorOpen,
    DoorClosed,
    RoomFloor,
    CaveWall,
    CaveFloor,
    Grass,
    GrassCrushed,
    Player,
    Npc(NpcType),
}

impl Tile {
    pub fn is_wall(&self) -> bool {
        matches!(self, Self::RoomWall | Self::DoorClosed | Self::DoorOpen | Self::CaveWall)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HitPoints {
    pub current: u32,
    pub max: u32,
}

impl HitPoints {
    pub fn new_full(max: u32) -> Self {
        Self { current: max, max }
    }
}

entity_table::declare_entity_module! {
    components {
        // Visibility
        opacity: u8,
        vision: vision_distance::Circle,
        light: Light<vision_distance::Circle>,
        is_dirty: bool,

        // Terrain
        solid: (),
        tile: Tile,
        grass_state: GrassState,
        door_state: DoorState,

        //Entity
        player: (),
        npc: Npc,
        enemy: (),

        hp: HitPoints,
        dead: (),
    }
}

pub use components::{Components, EntityData, EntityUpdate};
