use crate::prelude::*;
use gridbugs::entity_table::EntityAllocator;

mod action;
mod data;
mod query;
mod realtime;
mod spatial;
pub mod spawner;
mod sys;
mod visibility;

pub use action::*;
pub use data::*;
pub use query::*;
pub use realtime::*;
pub use spatial::*;
pub use sys::*;
pub use visibility::*;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub level: u32,
    #[serde(skip)]
    pub components: Components,
    pub spatial_table: SpatialTable,
    pub entity_allocator: EntityAllocator,
    pub realtime_components: realtime::RealtimeComponents,
}

impl World {
    pub fn new(size: Size, level: u32) -> Self {
        let components = Components::default();
        let spatial_table = SpatialTable::new(size);
        let entity_allocator = EntityAllocator::default();
        let realtime_components = realtime::RealtimeComponents::default();

        Self { level, entity_allocator, components, spatial_table, realtime_components }
    }

    pub fn size(&self) -> Size {
        self.spatial_table.grid_size()
    }

    pub fn clear_entity(&mut self, entity: Entity) {
        self.spatial_table.remove(entity);
        self.entity_allocator.free(entity);
        self.components.remove_entity(entity);
        self.realtime_components.remove_entity(entity);
    }

    pub fn animation_tick(&mut self, animation_context: &mut AnimationContext) {
        animation_context.tick(self);
    }
}
