use gridbugs::{chargrid::prelude::RenderCell, rgb_int::Rgba32};

use crate::{NpcType, Tile};

pub fn npc_char(npc_type: NpcType) -> char {
    match npc_type {
        NpcType::Orc => 'o',
        NpcType::Troll => 'T',
    }
}

pub fn npc_renderable(npc_type: NpcType) -> RenderCell {
    match npc_type {
        NpcType::Orc => RenderCell::BLANK.with_character(npc_char(npc_type)),
        NpcType::Troll => RenderCell::BLANK.with_character(npc_char(npc_type)),
    }
}

pub fn wall_renderable(tile: Tile, is_wall_below: bool) -> RenderCell {
    match tile {
        Tile::RoomWall => {
            if is_wall_below {
                RenderCell::BLANK.with_character(' ').with_background(Rgba32::new_grey(255))
            } else {
                RenderCell::BLANK
                    .with_character('▄')
                    .with_background(Rgba32::new_grey(255))
                    .with_foreground(Rgba32::new_grey(127))
            }
        }
        Tile::CaveWall => {
            if is_wall_below {
                RenderCell::BLANK.with_character(' ').with_background(Rgba32::new_rgb(125, 82, 44))
            } else {
                RenderCell::BLANK
                    .with_character('▄')
                    .with_background(Rgba32::new_rgb(125, 82, 44))
                    .with_foreground(Rgba32::new_rgb(68, 39, 14))
            }
        }
        _ => unreachable!("wall_character called on non-wall tile"),
    }
}

pub fn floor_renderable(tile: Tile) -> RenderCell {
    match tile {
        Tile::RoomFloor => RenderCell::BLANK.with_character('.').with_foreground(Rgba32::new_grey(127)),
        Tile::CaveFloor => {
            RenderCell::BLANK.with_character(',').with_foreground(Rgba32::new_rgb(125, 82, 44))
        }
        _ => unreachable!("wall_character called on non-wall tile"),
    }
}
