use crate::prelude::*;
use gridbugs::{
    chargrid::text::{StyledString, Text},
    visible_area_detection::CellVisibility,
};

mod character;
pub mod color;
mod examine;
mod menu;
pub mod text;
pub mod ui;

pub use character::*;
pub use examine::*;
pub use menu::*;
pub use text::*;
pub use ui::*;

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

#[derive(Clone, Copy)]
struct Remembered;
impl Tint for Remembered {
    fn tint(&self, rgba32: Rgba32) -> Rgba32 {
        let mean = rgba32.to_rgb24().weighted_mean_u16(rgb24::WeightsU16::new(1, 1, 1));
        Rgb24::new_grey(mean).saturating_scalar_mul_div(1, 2).to_rgba32(255)
    }
}

pub(crate) fn render_game_with_visibility(
    scope: &StateScope,
    offset: Coord,
    size: Size,
    ctx: Ctx,
    fb: &mut FrameBuffer,
) {
    let visibility_grid = scope.visibility_grid();
    for screen_coord in size.coord_iter_row_major() {
        let world_coord = offset + screen_coord;
        match visibility_grid.get_visibility(world_coord) {
            CellVisibility::Never => (),
            CellVisibility::Previous(data) => {
                let light_colour = AMBIENT_COL;
                render_cell(
                    scope,
                    screen_coord,
                    world_coord,
                    data,
                    true,
                    light_colour,
                    ctx_tint!(ctx, LightBlend { light_colour }),
                    fb,
                );
            }
            CellVisibility::Current { data, light_colour } => {
                log::info!("rendering");

                let light_colour = light_colour.unwrap_or(AMBIENT_COL);
                render_cell(
                    scope,
                    screen_coord,
                    world_coord,
                    data,
                    false,
                    light_colour,
                    ctx_tint!(ctx, LightBlend { light_colour }),
                    fb,
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn render_cell(
    scope: &StateScope,
    screen_coord: Coord,
    world_coord: Coord,
    cell: &VisibleCellData,
    remembered: bool,
    light_colour: Rgb24,
    ctx: Ctx,
    fb: &mut FrameBuffer,
) {
    cell.tiles.option_for_each_enumerate(|&tile, layer| {
        let render_cell = render_cell_from_tile(scope, tile, world_coord, remembered);
        let depth = layer_depth(layer);
        fb.set_cell_relative_to_ctx(ctx, screen_coord, depth, render_cell);
    });

    cell.realtime.iter().for_each(|entity| {
        let light_colour = Rgb24::new(light_colour.r, light_colour.g, light_colour.b);
        if let Some(tile) = entity.tile {
            let render_cell = render_cell_from_tile(scope, tile, entity.coord, remembered);
            fb.set_cell_relative_to_ctx(ctx, screen_coord, 1, render_cell);
        }

        if entity.particle {
            if let Some(fade) = entity.fade {
                let alpha = (255 - fade) / 10;
                let cell = RenderCell::BLANK
                    .with_background(Rgb24::new_grey(187).normalised_mul(light_colour).to_rgba32(alpha));
                fb.set_cell_relative_to_ctx(ctx, screen_coord, 1, cell);
            }
        }
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
        | Tile::DoorOpen
        | Tile::Reactor
        | Tile::Stairs => terrain_renderable(scope, tile, coord),

        // Entity
        Tile::Player | Tile::Npc(_) | Tile::Weapon(_) | Tile::Medkit => npc_renderable(tile, remembered),
        Tile::Bullet => RenderCell::BLANK.with_character('*').with_background(color::BULLET),
    }
}

fn layer_depth(layer: Layer) -> i8 {
    match layer {
        Layer::Floor => 0,
        Layer::Feature => 1,
        Layer::Item => 2,
        Layer::Character => 3,
    }
}
