use crate::prelude::*;
use gridbugs::entity_table::EntityAllocator;

mod action;
mod data;
mod query;
mod realtime;
mod render;
mod resource;
mod spatial;
pub mod spawner;
mod visibility;

pub use action::*;
pub use data::*;
pub use query::*;
pub use realtime::*;
pub use render::*;
pub use resource::*;
pub use spatial::*;
pub use visibility::*;

#[derive(Serialize, Deserialize)]
pub struct World {
    #[serde(skip)]
    pub resources: Resources,
    pub components: Components,
    pub spatial_table: SpatialTable,
    pub entity_allocator: EntityAllocator,
}

impl World {
    pub fn new(size: Size) -> Self {
        let entity_allocator = EntityAllocator::default();
        let components = Components::default();
        let spatial_table = SpatialTable::new(size);
        let resources = Resources::default();

        Self { entity_allocator, components, spatial_table, resources }
    }

    pub fn size(&self) -> Size {
        self.spatial_table.grid_size()
    }
}
