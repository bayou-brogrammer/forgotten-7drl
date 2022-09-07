use crate::prelude::*;

pub const CURSOR: Rgba32 = Rgba32::new(255, 255, 0, 64);
pub const STRIPE: Rgba32 = Rgba32::new_rgb(0xFF, 0xBE, 0x4C);
pub const SPACE_BACKGROUND: Rgba32 = Rgba32::new_rgb(0x00, 0x00, 0x38);
pub const MENU_BACKGROUND: Rgba32 = SPACE_BACKGROUND.saturating_scalar_mul_div(2, 3);

pub const WALL_TOP: Rgba32 = Rgba32::new_rgb(0x49, 0x2E, 0x00);
