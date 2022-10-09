use crate::prelude::*;

impl World {
    pub fn fire_laser(&mut self, character: Entity, target: Coord) {
        let character_coord = self.spatial_table.coord_of(character).unwrap();
        if character_coord == target {
            println!("Laser fired at self");
            return;
        }

        self.spawn_laser(character_coord, target)
    }
}
