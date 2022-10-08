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

#[derive(Default, Serialize, Deserialize)]
pub struct VisibleCellData {
    pub tiles: LayerTable<Option<Tile>>,
}

impl VisibleCellData {
    pub fn update(&mut self, world: &World, coord: Coord) {
        let layers = world.spatial_table.layers_at_checked(coord);
        self.tiles = layers.option_and_then(|&entity| world.components.tile.get(entity).cloned());
    }
}
