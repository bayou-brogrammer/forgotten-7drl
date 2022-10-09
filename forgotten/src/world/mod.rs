use crate::prelude::*;
use gridbugs::entity_table::EntityAllocator;

mod action;
mod data;
mod query;
mod realtime;
mod resource;
mod spatial;
pub mod spawner;
mod sys;
mod visibility;

pub use action::*;
pub use data::*;
pub use query::*;
pub use realtime::*;
pub use resource::*;
pub use spatial::*;
pub use sys::*;
pub use visibility::*;

#[derive(Serialize, Deserialize)]
pub struct World {
    #[serde(skip)]
    pub resources: Resources,
    pub components: Components,
    pub spatial_table: SpatialTable,
    pub entity_allocator: EntityAllocator,
    pub realtime_components: realtime::RealtimeComponents,
}

impl World {
    pub fn new(size: Size) -> Self {
        let resources = Resources::default();
        let components = Components::default();
        let spatial_table = SpatialTable::new(size);
        let entity_allocator = EntityAllocator::default();
        let realtime_components = realtime::RealtimeComponents::default();

        Self { entity_allocator, components, spatial_table, resources, realtime_components }
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
