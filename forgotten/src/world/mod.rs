use crate::prelude::*;
use gridbugs::{entity_table::EntityAllocator, shadowcast::vision_distance::Circle};

mod action;
mod components;
mod realtime;
mod spatial;

pub use action::*;
pub use components::*;
pub use realtime::*;
pub use spatial::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub wrld: shipyard::World,
    pub components: Components,
    pub spatial_table: SpatialTable,
    pub entity_allocator: EntityAllocator,
}

impl World {
    pub fn new(size: Size) -> Self {
        let entity_allocator = EntityAllocator::default();
        let components = Components::default();
        let spatial_table = SpatialTable::new(size);

        let wrld = shipyard::World::new();

        Self { entity_allocator, components, spatial_table, wrld }
    }

    pub fn size(&self) -> Size {
        self.spatial_table.grid_size()
    }

    pub fn entity_coord(&self, entity: Entity) -> Option<Coord> {
        self.spatial_table.coord_of(entity)
    }

    pub fn get_opacity_at_coord(&self, coord: Coord) -> u8 {
        self.spatial_table
            .layers_at(coord)
            .and_then(|c| c.feature)
            .and_then(|e| self.components.opacity.get(e).cloned())
            .unwrap_or(0)
    }

    pub fn all_lights_by_coord(&self) -> impl '_ + Iterator<Item = (Coord, &Light)> {
        self.components.light.iter().filter_map(move |(entity, light)| {
            self.spatial_table.coord_of(entity).map(|coord| (coord, light))
        })
    }

    pub fn insert_entity_data(&mut self, location: Location, entity_data: EntityData) -> Entity {
        let entity = self.entity_allocator.alloc();
        self.spatial_table.update(entity, location).unwrap();
        self.components.insert_entity_data(entity, entity_data);
        entity
    }
}

// Spawns
impl World {
    pub fn make_player() -> EntityData {
        EntityData {
            tile: Some(Tile::Player),
            light: Some(Light {
                colour: Rgb24::new_grey(63),
                vision_distance: Circle::new_squared(90),
                diminish: Rational { numerator: 1, denominator: 4 },
            }),
            ..Default::default()
        }
    }

    pub fn spawn_floor(&mut self, coord: Coord) -> Entity {
        let entity = self.entity_allocator.alloc();
        self.spatial_table.update(entity, Location { coord, layer: Some(Layer::Floor) }).unwrap();
        self.components.tile.insert(entity, Tile::Floor);
        entity
    }

    pub fn spawn_wall(&mut self, coord: Coord) -> Entity {
        let entity = self.entity_allocator.alloc();
        self.spatial_table.update(entity, Location { coord, layer: Some(Layer::Feature) }).unwrap();
        self.components.tile.insert(entity, Tile::Wall);
        self.components.solid.insert(entity, ());
        self.components.opacity.insert(entity, 255);
        entity
    }

    pub fn spawn_light(&mut self, coord: Coord, colour: Rgb24) -> Entity {
        let entity = self.entity_allocator.alloc();
        self.spatial_table.update(entity, Location { coord, layer: None }).unwrap();
        self.components.light.insert(
            entity,
            Light {
                colour,
                vision_distance: Circle::new_squared(200),
                diminish: Rational { numerator: 1, denominator: 10 },
            },
        );
        entity
    }
}
