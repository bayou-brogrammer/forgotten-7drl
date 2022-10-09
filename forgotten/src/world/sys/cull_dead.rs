use crate::prelude::*;

pub struct PlayerDied(pub EntityData);

impl World {
    pub fn cull_dead(&mut self, agents: &mut ComponentTable<Agent>) {
        for entity in self.components.dead.entities().collect::<Vec<_>>() {
            if self.components.player.get(entity).is_some() {
            } else {
                agents.remove(entity);
                self.components.remove_entity(entity);
                self.spatial_table.remove(entity);
                self.entity_allocator.free(entity);
            }
        }
    }
}
