mod cull_dead;
use crate::{Agent, AnimationContext, World};
pub use cull_dead::*;
use gridbugs::entity_table::ComponentTable;

impl World {
    pub fn run_systems(
        &mut self,
        agents: &mut ComponentTable<Agent>,
        animation_context: &mut AnimationContext,
    ) {
        self.cull_dead(agents);
        self.animation_tick(animation_context);
    }
}
