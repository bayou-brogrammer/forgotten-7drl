pub use crate::prelude::*;

impl Game {
    pub fn update_behaviour(&mut self) {
        self.behavior_context.update(self.player_entity, &self.world);
    }

    pub fn prime_npcs(&mut self) {
        self.update_behaviour();
    }

    pub fn npc_turn(&mut self) {
        self.update_behaviour();

        for (entity, agent) in self.agents.iter_mut() {
            if !self.world.entity_exists(entity) {
                self.world.components.dead.insert(entity, ());
                continue;
            }

            match agent.act(entity, &self.world, self.player_entity, &mut self.behavior_context) {
                NpcAction::Wait => (),
                NpcAction::Alert(direction) => {
                    self.world.alert_nearby_entities(entity, direction, self.player_entity)
                }
                NpcAction::Walk(direction) => {
                    let _ = self.world.character_walk_in_direction(entity, direction);
                }
                NpcAction::FireBullet(direction) => {
                    let agent_coord = self.world.entity_coord(entity).unwrap();

                    self.world.character_fire_bullet(
                        entity,
                        agent_coord + (direction.coord() * 100),
                        RangedWeaponSlot::Slot1,
                    );
                }
            }
        }
    }
}
