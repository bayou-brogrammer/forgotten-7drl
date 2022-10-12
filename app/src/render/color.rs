use gridbugs::rgb_int::Rgba32;

pub const CURSOR: Rgba32 = Rgba32::new(255, 255, 0, 64);
pub const STRIPE: Rgba32 = Rgba32::new_rgb(0xFF, 0xBE, 0x4C);
pub const SPACE_BACKGROUND: Rgba32 = Rgba32::new_rgb(0x00, 0x00, 0x38);
pub const MENU_BACKGROUND: Rgba32 = SPACE_BACKGROUND.saturating_scalar_mul_div(2, 3);

// Base Colors
pub const RED: Rgba32 = Rgba32::new_rgb(255, 0, 0);
pub const BLACK: Rgba32 = Rgba32::new_grey(0);
pub const YELLOW: Rgba32 = Rgba32::new_rgb(255, 255, 0);
pub const LIGHT_GREY: Rgba32 = Rgba32::new_grey(127);
pub const WHITE: Rgba32 = Rgba32::new_grey(255);
pub const BLUE_VIOLET: Rgba32 = Rgba32::new_rgb(138, 43, 226);
pub const LIGHT_RED: Rgba32 = HEALTH.saturating_scalar_mul_div(2, 3);

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
pub const WATER_FG: Rgba32 = Rgba32::new_rgb(0, 127, 187);
pub const WATER_BG: Rgba32 = Rgba32::new_rgb(0, 63, 127);
pub const REACTOR: Rgba32 = Rgba32::new_rgb(255, 132, 0);
pub const STAIRS: Rgba32 = Rgba32::new_rgb(0, 255, 0);

// Entity Colors
pub const PLAYER: Rgba32 = YELLOW;
pub const MINIBOT: Rgba32 = Rgba32::new_rgb(0, 187, 0);
pub const SECBOT: Rgba32 = Rgba32::new_rgb(187, 0, 0);
pub const ROBOCOP: Rgba32 = Rgba32::new_rgb(187, 0, 0);
pub const DOOMBOT: Rgba32 = Rgba32::new_rgb(187, 0, 0);

// Ability Colors
pub const LASER: Rgba32 = Rgba32::new_rgb(0, 255, 0);
pub const HEALTH: Rgba32 = Rgba32::new_rgb(255, 0, 0);
pub const SHOCK: Rgba32 = Rgba32::new_rgb(255, 255, 31);

// Weapon Colors
pub const LEECH: Rgba32 = Rgba32::new_rgb(75, 255, 0);
pub const OXYGEN: Rgba32 = Rgba32::new_rgb(127, 127, 255);
pub const BULLET: Rgba32 = Rgba32::new_grey(0);
pub const GAUS: Rgba32 = Rgba32::new_rgb(127, 0, 255);
pub const PLASMA: Rgba32 = Rgba32::new_rgb(0x00, 0xFF, 0xFF);
pub const CHAINSAW: Rgba32 = Rgba32::new_rgb(0x7a, 0x6a, 0x00);
pub const CREDIT_FOREGROUND: Rgba32 = Rgba32::new_rgb(0, 127, 127);

// pub const MEDKIT: Rgba32 = Rgba32::new_grey(200);
// pub const MEDKIT_TOP: Rgba32 = Rgba32::new_grey(150);
// pub const MAP_FOREGROUND: Rgba32 = Rgba32::new_rgb(0, 63, 0);
// pub const MAP_BACKGROUND: Rgba32 = Rgba32::new_rgb(0, 255, 0);
