use crate::World;
use gridbugs::entity_table_realtime::{declare_realtime_entity_module, ContextContainsRealtimeComponents};

pub mod animation;
pub mod movement;

pub use animation::*;
pub use movement::*;

pub struct Context<'a> {
    world: &'a mut World,
}

impl<'a> ContextContainsRealtimeComponents for Context<'a> {
    type Components = RealtimeComponents;

    fn components_mut(&mut self) -> &mut Self::Components {
        &mut self.world.realtime_components
    }
}

declare_realtime_entity_module! {
  components<'a>[Context<'a>] {
      movement: MovementState,
  }
}

pub use components::RealtimeComponents;
