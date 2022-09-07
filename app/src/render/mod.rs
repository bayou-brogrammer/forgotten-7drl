use crate::prelude::*;
use gridbugs::chargrid::text::{StyledString, Text};

mod menu;
pub mod text;

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
    scope: &WitnessScope,
    offset: Coord,
    size: Size,
    ctx: Ctx,
    fb: &mut FrameBuffer,
) {
    let visibility_grid = scope.visibility_grid();
    let vis_count = visibility_grid.count();
    for screen_coord in size.coord_iter_row_major() {
        let world_coord = offset + screen_coord;
        if let Some(visibility_cell) = visibility_grid.get_cell(world_coord) {
            match visibility_cell.visibility(vis_count) {
                CellVisibility::CurrentlyVisibleWithLightColour(Some(light_colour)) => render_visibile(
                    screen_coord,
                    visibility_cell,
                    ctx_tint!(ctx, LightBlend { light_colour }),
                    fb,
                ),
                CellVisibility::PreviouslyVisible => {
                    render_remembered(screen_coord, visibility_cell, ctx, fb)
                }
                CellVisibility::NeverVisible | CellVisibility::CurrentlyVisibleWithLightColour(None) => (),
            }
        }
    }
}

fn render_visibile(coord: Coord, cell: &VisibilityCell, ctx: Ctx, fb: &mut FrameBuffer) {
    let mut render_tile = |_entity, tile| {
        let ch = match tile {
            Tile::Floor => '.',
            Tile::Wall => '█',
            Tile::Player => '@',
        };
        fb.set_cell_relative_to_ctx(
            ctx,
            coord,
            0,
            RenderCell::default().with_character(ch).with_foreground(Rgba32::new_grey(255)),
        );
    };

    let tile_layers = cell.tile_layers();
    if let Some(EntityTile { entity, tile }) = tile_layers.floor {
        render_tile(entity, tile);
    }
    if let Some(EntityTile { entity, tile }) = tile_layers.feature {
        render_tile(entity, tile);
    }
    if let Some(EntityTile { entity, tile }) = tile_layers.item {
        render_tile(entity, tile);
    }
    if let Some(EntityTile { entity, tile }) = tile_layers.character {
        render_tile(entity, tile);
    }
}

fn render_remembered(coord: Coord, cell: &VisibilityCell, ctx: Ctx, fb: &mut FrameBuffer) {
    let tile_layers = cell.tile_layers();
    if let Some(EntityTile { tile, .. }) = tile_layers.feature {
        if tile == Tile::Wall {
            fb.set_cell_relative_to_ctx(
                ctx,
                coord,
                0,
                RenderCell::default().with_character('▒').with_foreground(Rgba32::new_grey(127)),
            );
        }
    }
}
