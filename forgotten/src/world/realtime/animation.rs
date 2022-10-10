use gridbugs::entity_table_realtime;

use crate::prelude::*;

pub const ANIMATION_FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / 60);

pub fn period_per_frame(num_per_frame: u32) -> Duration {
    ANIMATION_FRAME_DURATION / num_per_frame
}

#[derive(Default)]
pub struct AnimationContext {
    realtime_entities: Vec<Entity>,
}

impl Serialize for AnimationContext {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        ().serialize(s)
    }
}

impl<'a> Deserialize<'a> for AnimationContext {
    fn deserialize<D: serde::Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(d)?;
        Ok(Self::default())
    }
}

impl AnimationContext {
    pub fn tick(&mut self, world: &mut World) {
        self.realtime_entities.extend(world.components.realtime.entities());

        let mut context = Context { world };
        for entity in self.realtime_entities.drain(..) {
            entity_table_realtime::process_entity_frame(entity, ANIMATION_FRAME_DURATION, &mut context);
        }
    }
}
