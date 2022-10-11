use crate::prelude::*;
use gridbugs::{
    coord_2d::{Coord, Size},
    visible_area_detection::{vision_distance, Light, World as VisibleWorld},
};
use serde::{Deserialize, Serialize};

impl VisibleWorld for World {
    type VisionDistance = vision_distance::Circle;

    fn size(&self) -> Size {
        self.spatial_table.grid_size()
    }

    fn get_opacity(&self, coord: Coord) -> u8 {
        if let Some(&Layers { feature: Some(feature_entity), .. }) = self.spatial_table.layers_at(coord) {
            self.components.opacity.get(feature_entity).cloned().unwrap_or(0)
        } else {
            0
        }
    }

    fn for_each_light_by_coord<F: FnMut(Coord, &Light<Self::VisionDistance>)>(&self, mut f: F) {
        self.components.light.iter().for_each(|(entity, light)| {
            if let Some((c, l)) = self.spatial_table.coord_of(entity).map(|coord| (coord, light)) {
                f(c, l)
            }
        });
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RealTimeEntity {
    pub coord: Coord,
    pub particle: bool,
    pub fade: Option<u8>,
    pub tile: Option<Tile>,
    pub layer: Option<Layer>,
    pub colour_hint: Option<Rgb24>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct VisibleCellData {
    pub tiles: LayerTable<Option<Tile>>,
    pub realtime: Vec<RealTimeEntity>,
}

impl VisibleCellData {
    pub fn update(&mut self, world: &World, coord: Coord) {
        let layers = world.spatial_table.layers_at_checked(coord);
        self.tiles = layers.option_and_then(|&entity| world.components.tile.get(entity).cloned());

        let tile_component = &world.components.tile;
        let spatial_table = &world.spatial_table;
        let realtime_fade_component = &world.realtime_components.fade;
        let colour_hint_component = &world.components.colour_hint;
        let particle_component = &world.components.particle;
        let realtime_component = &world.components.realtime;

        self.realtime = realtime_component
            .iter()
            .filter_map(move |(entity, &())| {
                let location = spatial_table.location_of(entity)?;
                if location.coord == coord {
                    let fade = realtime_fade_component.get(entity).and_then(|f| f.fading());
                    let tile = tile_component.get(entity).cloned();
                    let colour_hint = colour_hint_component.get(entity).cloned();
                    let particle = particle_component.contains(entity);
                    Some(RealTimeEntity {
                        coord: location.coord,
                        layer: location.layer,
                        tile,
                        fade,
                        colour_hint,
                        particle,
                    })
                } else {
                    None
                }
            })
            .collect();
    }
}
