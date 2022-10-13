use forgotten_game::{Coord, NpcType, StateScope};
use gridbugs::chargrid::prelude::RenderCell;

use crate::{color::*, Tile};

pub fn npc_renderable(tile: Tile, remembered: bool) -> RenderCell {
    if remembered {
        RenderCell::BLANK
    } else {
        match tile {
            Tile::Player => RenderCell::BLANK.with_character('@').with_foreground(PLAYER).with_bold(true),
            Tile::Npc(npc_type) => match npc_type {
                NpcType::MiniBot => {
                    RenderCell::BLANK.with_character('c').with_foreground(MINIBOT).with_bold(true)
                }
                NpcType::SecBot => {
                    RenderCell::BLANK.with_character('Č').with_foreground(SECBOT).with_bold(true)
                }
                NpcType::RoboCop => {
                    RenderCell::BLANK.with_character('Ĝ').with_foreground(ROBOCOP).with_bold(true)
                }
                NpcType::DoomBot => {
                    RenderCell::BLANK.with_character('Œ').with_foreground(DOOMBOT).with_bold(true)
                }
            },

            _ => unreachable!("npc_renderable called with non-npc tile"),
        }
    }
}

pub fn terrain_renderable(scope: &StateScope, tile: Tile, coord: Coord) -> RenderCell {
    match tile {
        Tile::DoorClosed => {
            RenderCell::BLANK.with_character('+').with_background(LIGHT_GREY).with_foreground(WHITE)
        }
        Tile::DoorOpen => {
            RenderCell::BLANK.with_character('-').with_background(LIGHT_GREY).with_foreground(WHITE)
        }
        Tile::Floor | Tile::CaveFloor | Tile::Water | Tile::Grass | Tile::GrassCrushed => {
            floor_renderable(tile)
        }
        Tile::Wall | Tile::CaveWall => {
            let is_wall_below = scope.0.is_wall_known_at(coord + Coord::new(0, 1));
            wall_renderable(tile, is_wall_below)
        }
        Tile::Reactor => RenderCell::BLANK.with_character('☼').with_foreground(REACTOR),
        Tile::Stairs => RenderCell::BLANK.with_character('>').with_foreground(STAIRS),
        _ => unreachable!("Tried to render a non-terrain tile as terrain: {:?}", tile),
    }
}

pub fn wall_renderable(tile: Tile, is_wall_below: bool) -> RenderCell {
    match tile {
        Tile::Wall => {
            if is_wall_below {
                RenderCell::BLANK.with_character(' ').with_background(WHITE)
            } else {
                RenderCell::BLANK.with_character('▄').with_background(WHITE).with_foreground(LIGHT_GREY)
            }
        }
        Tile::CaveWall => {
            if is_wall_below {
                RenderCell::BLANK.with_character(' ').with_background(CAVE_WALL_BG)
            } else {
                RenderCell::BLANK
                    .with_character('▄')
                    .with_background(CAVE_WALL_BG)
                    .with_foreground(CAVE_WALL_FG)
            }
        }
        _ => unreachable!("wall_renderable called on non-wall tile"),
    }
}

pub fn floor_renderable(tile: Tile) -> RenderCell {
    match tile {
        Tile::Grass => RenderCell::BLANK.with_character('"').with_foreground(GRASS),
        Tile::Floor => RenderCell::BLANK.with_character('.').with_foreground(LIGHT_GREY),
        Tile::CaveFloor => RenderCell::BLANK.with_character(',').with_foreground(CAVE_WALL_BG),
        Tile::GrassCrushed => RenderCell::BLANK.with_character('\'').with_foreground(GRASS_CRUSHED),
        Tile::Water => {
            RenderCell::BLANK.with_character('≈').with_foreground(WATER_FG).with_background(WATER_BG)
        }
        _ => unreachable!("floor_renderable called on non-wall tile"),
    }
}

pub fn item_renderable(tile: Tile) -> RenderCell {
    match tile {
        Tile::Weapon(weapon_type) => match weapon_type {
            forgotten_game::WeaponType::BareHands => RenderCell::BLANK,
            forgotten_game::WeaponType::CattleProd => {
                RenderCell::BLANK.with_character('Δ').with_foreground(YELLOW).with_bold(true)
            }
            forgotten_game::WeaponType::Chainsaw => {
                RenderCell::BLANK.with_character('Э').with_foreground(CHAINSAW).with_bold(true)
            }
            forgotten_game::WeaponType::Railgun => {
                RenderCell::BLANK.with_character('Я').with_foreground(PLASMA).with_bold(true)
            }
            forgotten_game::WeaponType::Leecher => {
                RenderCell::BLANK.with_character('ł').with_foreground(LEECH).with_bold(true)
            }
            forgotten_game::WeaponType::FiftyCal => {
                RenderCell::BLANK.with_character('ξ').with_foreground(GAUS).with_bold(true)
            }
            forgotten_game::WeaponType::Pistol => {
                RenderCell::BLANK.with_character('┌').with_foreground(OXYGEN).with_bold(true)
            }
            forgotten_game::WeaponType::Rifle => {
                RenderCell::BLANK.with_character('√').with_foreground(LASER).with_bold(true)
            }
        },

        Tile::Medkit => RenderCell::BLANK
            .with_character('†')
            .with_foreground(HEALTH)
            .with_background(MEDKIT_TOP)
            .with_bold(true),
        Tile::Upgrade => RenderCell::BLANK
            .with_character('Ū')
            .with_foreground(UPGRADE_FOREGROUND)
            .with_background(UPGRADE_BACKGROUND)
            .with_bold(true),
        Tile::Credit1 => {
            RenderCell::BLANK.with_character('1').with_foreground(CREDIT_FOREGROUND).with_bold(true)
        }
        Tile::Credit2 => {
            RenderCell::BLANK.with_character('2').with_foreground(CREDIT_FOREGROUND).with_bold(true)
        }
        Tile::Credit3 => {
            RenderCell::BLANK.with_character('3').with_foreground(CREDIT_FOREGROUND).with_bold(true)
        }

        _ => unreachable!("item_renderable called on non-wall tile"),
    }
}
