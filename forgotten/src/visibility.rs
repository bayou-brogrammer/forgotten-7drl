use crate::prelude::*;
use gridbugs::{spatial_table::Coord, visible_area_detection::CellVisibility};

pub const AMBIENT_COL: Rgb24 = Rgb24::new_grey(75);

impl Game {
    pub fn update_visibility(&mut self) {
        let update_fn = |data: &mut VisibleCellData, coord| data.update(&self.world, coord);

        if self.config.omniscient {
            self.visibility_grid.update_omniscient_custom(AMBIENT_COL, &self.world, update_fn);
        } else {
            let player_vision = *self.world.components.vision.get(self.player_entity).unwrap();
            let player_coord = self.player_coord();
            self.visibility_grid.update_custom(
                AMBIENT_COL,
                &self.world,
                player_vision,
                player_coord,
                update_fn,
            );
        }
    }

    /// Returns an iterator over each coordinate of the world, along with the visibility of each
    /// corresponding cell
    pub fn enumerate_cell_visibility(
        &self,
    ) -> impl Iterator<Item = (Coord, CellVisibility<&VisibleCellData>)> {
        self.visibility_grid.enumerate()
    }
}
