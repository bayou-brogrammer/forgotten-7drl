use crate::NpcType;
use gridbugs::rgb_int::Rgba32;

pub const CURSOR: Rgba32 = Rgba32::new(255, 255, 0, 64);
pub const STRIPE: Rgba32 = Rgba32::new_rgb(0xFF, 0xBE, 0x4C);
pub const SPACE_BACKGROUND: Rgba32 = Rgba32::new_rgb(0x00, 0x00, 0x38);
pub const MENU_BACKGROUND: Rgba32 = SPACE_BACKGROUND.saturating_scalar_mul_div(2, 3);

pub const WALL_TOP: Rgba32 = Rgba32::new_rgb(0x49, 0x2E, 0x00);

// Base Colors
pub const RED: Rgba32 = Rgba32::new_rgb(255, 0, 0);
pub const BLACK: Rgba32 = Rgba32::new_grey(0);
pub const YELLOW: Rgba32 = Rgba32::new_rgb(255, 255, 0);
pub const LIGHT_GREY: Rgba32 = Rgba32::new_grey(127);
pub const WHITE: Rgba32 = Rgba32::new_grey(255);

// Terrain Colors
pub const FLOOR: Rgba32 = LIGHT_GREY;
pub const DOOR_FG: Rgba32 = LIGHT_GREY;
pub const DOOR_BG: Rgba32 = WHITE;
pub const GRASS: Rgba32 = Rgba32::new_rgb(0, 187, 63);
pub const GRASS_CRUSHED: Rgba32 = Rgba32::new_rgb(0, 127, 63);
pub const CAVE_WALL_FG: Rgba32 = Rgba32::new_rgb(68, 39, 14);
pub const CAVE_WALL_BG: Rgba32 = Rgba32::new_rgb(125, 82, 44);
pub const ROOM_WALL_FG: Rgba32 = WHITE;
pub const ROOM_WALL_BG: Rgba32 = LIGHT_GREY;

// Entity Colors
pub const PLAYER: Rgba32 = YELLOW;
pub const ORC: Rgba32 = Rgba32::new_rgb(0, 187, 0);
pub const TROLL: Rgba32 = Rgba32::new_rgb(187, 0, 0);

// Item Colors
pub const HEALTH_POTION: Rgba32 = Rgba32::new_rgb(255, 0, 255);
pub const FIREBALL_SCROLL: Rgba32 = Rgba32::new_rgb(255, 127, 0);
pub const CONFUSION_SCROLL: Rgba32 = Rgba32::new_rgb(187, 0, 255);
pub const SWORD: Rgba32 = Rgba32::new_rgb(187, 187, 187);
pub const STAFF: Rgba32 = Rgba32::new_rgb(187, 127, 255);
pub const ARMOUR: Rgba32 = Rgba32::new_rgb(127, 127, 127);
pub const ROBE: Rgba32 = Rgba32::new_rgb(127, 127, 187);

pub fn npc_colour(npc_type: NpcType) -> Rgba32 {
    match npc_type {
        NpcType::Orc => ORC,
        NpcType::Troll => TROLL,
    }
}
