use crate::prelude::*;

impl World {
    pub fn damage_character(&mut self, character: Entity, hit_points_to_lose: u32) {
        let hit_points =
            self.components.hp.get_mut(character).expect("character lacks hit_points");

        if hit_points_to_lose >= hit_points.current {
            hit_points.current = 0;
            self.character_die(character);
        } else {
            hit_points.current -= hit_points_to_lose;
        }
    }

    fn character_die(&mut self, character: Entity) {
        self.components.dead.insert(character, ());
    }
}
