use crate::prelude::*;

pub struct ToRenderEntity {
    pub tile: Tile,
    // pub blood: bool,
    pub coord: Coord,
    // pub fade: Option<u8>,
    pub layer: Option<Layer>,
    // pub ignore_lighting: bool,
    // pub colour_hint: Option<Rgb24>,
}

impl World {
    pub fn to_render_entities(&self) -> impl '_ + Iterator<Item = ToRenderEntity> {
        let tile_component = &self.components.tile;

        let spatial_table = &self.spatial_table;
        tile_component.iter().filter_map(move |(entity, &tile)| {
            spatial_table.location_of(entity).map(|location| ToRenderEntity {
                tile,
                // fade: todo!(),
                // blood: todo!(),
                // colour_hint: todo!(),
                coord: location.coord,
                layer: location.layer,
                // ignore_lighting: todo!(),
            })
        })
    }
}
