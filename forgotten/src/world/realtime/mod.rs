use crate::World;
use gridbugs::entity_table_realtime::{declare_realtime_entity_module, ContextContainsRealtimeComponents};

pub mod animation;
pub mod fade;
pub mod light_colour_fade;
pub mod movement;
pub mod particle;

pub use animation::*;
pub use fade::*;
pub use light_colour_fade::*;
pub use movement::*;
pub use particle::*;

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
      fade: FadeState,
      movement: MovementState,
      particle_emitter: ParticleEmitterState,
      light_colour_fade: LightColourFadeState,
    }
}

pub use components::RealtimeComponents;
