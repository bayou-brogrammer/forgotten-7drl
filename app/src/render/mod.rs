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
                render_cell(scope, coord, data, true, dim_ctx, fb);
            }
            CellVisibility::Current { data, light_colour } => {
                let light_colour = light_colour.unwrap_or(Rgb24::new_grey(255));
                render_cell(scope, coord, data, false, ctx_tint!(ctx, LightBlend { light_colour }), fb);
            }
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

fn render_cell(
    scope: &StateScope,
    coord: Coord,
    cell: &VisibleCellData,
    remembered: bool,
    ctx: Ctx,
    fb: &mut FrameBuffer,
) {
    cell.tiles.option_for_each_enumerate(|&tile, layer| {
        let render_cell = render_cell_from_tile(scope, tile, coord, remembered);
        let depth = layer_depth(layer);
        fb.set_cell_relative_to_ctx(ctx, coord, depth, render_cell);
    });
}

/// Associate each tile with a description of how to render it
fn render_cell_from_tile(scope: &StateScope, tile: Tile, coord: Coord, remembered: bool) -> RenderCell {
    match tile {
        // Terrain
        Tile::Wall
        | Tile::CaveWall
        | Tile::Floor
        | Tile::CaveFloor
        | Tile::Water
        | Tile::Grass
        | Tile::GrassCrushed
        | Tile::DoorClosed
        | Tile::DoorOpen => terrain_renderable(scope, tile, coord),

        // Entity
        Tile::Player | Tile::Npc(_) | Tile::Weapon(_) => npc_renderable(tile, remembered),
    }
}

fn layer_depth(layer: Layer) -> i8 {
    match layer {
        Layer::Character => 2,
        Layer::Feature => 1,
        Layer::Floor => 0,
    }
}
