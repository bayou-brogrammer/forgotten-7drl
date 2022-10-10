use crate::prelude::*;

impl World {
    pub fn apply_stun(&mut self, entity: Entity, stun_percentage: u8) -> bool {
        if crate::rng::range(0..100) < stun_percentage {
            self.components.stunned.insert(entity, Stunned { turns: 1 });
            true
        } else {
            false
        }
    }

    pub fn reduce_stun(&mut self, entity: Entity) {
        if let Some(stun) = self.components.stunned.get_mut(entity) {
            stun.turns -= 1;

            if stun.turns == 0 {
                self.components.stunned.remove(entity);
            }
        }
    }

    pub fn fire_laser(&mut self, character: Entity, target: Coord) {
        let character_coord = self.spatial_table.coord_of(character).unwrap();
        if character_coord == target {
            return;
        }

        self.spawn_laser(character_coord, target)
    }
}
