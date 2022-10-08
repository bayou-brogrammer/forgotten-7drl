use crate::prelude::*;
use gridbugs::{
    chargrid::text::{StyledString, Text},
    visible_area_detection::CellVisibility,
};

mod character;
pub mod color;
mod menu;
pub mod text;

pub use character::*;
pub use menu::*;
pub use text::*;

#[derive(Clone, Copy)]
pub struct LightBlend {
    light_colour: Rgb24,
}

impl Tint for LightBlend {
    fn tint(&self, rgba32: Rgba32) -> Rgba32 {
        rgba32
            .to_rgb24()
            .normalised_mul(self.light_colour)
            .saturating_add(self.light_colour.saturating_scalar_mul_div(1, 10))
            .to_rgba32(255)
    }
}

pub(crate) fn render_game_with_visibility(
    scope: &StateScope,
    offset: Coord,
    size: Size,
    ctx: Ctx,
    fb: &mut FrameBuffer,
) {
    for (coord, visibility) in scope.0.enumerate_cell_visibility() {
        match visibility {
            CellVisibility::Never => (),
            CellVisibility::Previous(data) => {
                let dim_ctx = ctx.with_tint(&|colour: Rgba32| colour.saturating_scalar_mul_div(1, 2));
                render_cell(scope, coord, data, dim_ctx, fb);
            }
            CellVisibility::Current { data, .. } => render_cell(scope, coord, data, ctx, fb),
        }
    }

    // let visibility_grid = *scope.visibility_grid();
    // let vis_count = visibility_grid.count();
    // for screen_coord in size.coord_iter_row_major() {
    //     let world_coord = offset + screen_coord;
    //     if let Some(visibility_cell) = visibility_grid.get_cell(world_coord) {
    //         match visibility_cell.visibility(vis_count) {
    //             CellVisibility::CurrentlyVisibleWithLightColour(Some(light_colour)) => render_visibile(
    //                 screen_coord,
    //                 visibility_cell,
    //                 ctx_tint!(ctx, LightBlend { light_colour }),
    //                 fb,
    //             ),
    //             CellVisibility::PreviouslyVisible => {
    //                 let dim_ctx = ctx.with_tint(&|colour: Rgba32| colour.saturating_scalar_mul_div(1, 2));
    //                 render_remembered(screen_coord, visibility_cell, dim_ctx, fb)
    //             }
    //             CellVisibility::NeverVisible | CellVisibility::CurrentlyVisibleWithLightColour(None) => (),
    //         }
    //     }
    // }
}

fn render_cell(scope: &StateScope, coord: Coord, cell: &VisibleCellData, ctx: Ctx, fb: &mut FrameBuffer) {
    // Render Map
    cell.tiles.option_for_each_enumerate(|&tile, layer| {
        let render_cell = render_cell_from_tile(scope, tile, coord);
        let depth = layer_depth(layer);
        fb.set_cell_relative_to_ctx(ctx, coord, depth, render_cell);
    });
}

/// Associate each tile with a description of how to render it
fn render_cell_from_tile(scope: &StateScope, tile: Tile, coord: Coord) -> RenderCell {
    match tile {
        // Terrain
        Tile::DoorClosed => {
            RenderCell::BLANK.with_character('+').with_background(LIGHT_GREY).with_foreground(WHITE)
        }
        Tile::DoorOpen => {
            RenderCell::BLANK.with_character('-').with_background(LIGHT_GREY).with_foreground(WHITE)
        }
        Tile::Grass => RenderCell::BLANK.with_character('"').with_foreground(GRASS),
        Tile::GrassCrushed => RenderCell::BLANK.with_character('\'').with_foreground(GRASS_CRUSHED),
        Tile::RoomFloor | Tile::CaveFloor => floor_renderable(tile),
        Tile::RoomWall | Tile::CaveWall => {
            let is_wall_below = scope.0.is_wall_known_at(coord + Coord::new(0, 1));
            wall_renderable(tile, is_wall_below)
        }

        // Entity
        Tile::Player => RenderCell::BLANK.with_character('@').with_foreground(PLAYER).with_bold(true),
        Tile::Npc(npc_type) => RenderCell::BLANK
            .with_character(npc_char(npc_type))
            .with_foreground(npc_colour(npc_type))
            .with_bold(true),
    }
}

fn layer_depth(layer: Layer) -> i8 {
    match layer {
        Layer::Character => 2,
        Layer::Feature => 1,
        Layer::Floor => 0,
    }
}
