use crate::prelude::*;
use gridbugs::{
    coord_2d::Coord,
    entity_table::{entity_data, Entity},
    visible_area_detection::vision_distance,
};

impl World {
    /// Helper method to spawn an entity at a location
    fn spawn_entity<L: Into<Location>>(&mut self, location: L, entity_data: EntityData) -> Entity {
        let entity = self.entity_allocator.alloc();
        let location @ Location { layer, coord } = location.into();
        if let Err(e) = self.spatial_table.update(entity, location) {
            panic!("{:?}: There is already a {:?} at {:?}", e, layer, coord);
        }
        self.components.insert_entity_data(entity, entity_data);
        entity
    }

    pub fn spawn_player(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                vision: vision_distance::Circle::new(200),
                hp: HitPoints::new_full(10),
                player: (),
                tile: Tile::Player,
            },
        )
    }

    pub fn spawn_room_wall(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Feature),
            entity_data! {
                tile: Tile::RoomWall,
                solid: (),
                opacity: 255,
            },
        );
    }

    pub fn spawn_cave_wall(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Feature),
            entity_data! {
                tile: Tile::CaveWall,
                solid: (),
                opacity: 255,
            },
        );
    }

    pub fn spawn_door(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Feature),
            entity_data! {
                tile: Tile::DoorClosed,
                door_state: DoorState::Closed,
                solid: (),
                opacity: 255,
            },
        );
    }

    pub fn spawn_room_floor(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Floor),
            entity_data! {
                tile: Tile::RoomFloor,
            },
        );
    }

    pub fn spawn_cave_floor(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Floor),
            entity_data! {
                tile: Tile::CaveFloor,
            },
        );
    }

    pub fn spawn_grass(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Feature),
            entity_data! {
                tile: Tile::Grass,
                opacity: 128,
                grass_state: GrassState::Normal,
            },
        );
    }

    pub fn spawn_orc(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                npc: Npc {
                    disposition: Disposition::Hostile,
                },
                enemy: (),
                hp: HitPoints::new_full(5),
                tile: Tile::Npc(NpcType::Orc),
            },
        )
    }

    pub fn spawn_troll(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                npc: Npc {
                    disposition: Disposition::Hostile,
                },
                enemy: (),
                hp: HitPoints::new_full(8),
                tile: Tile::Npc(NpcType::Troll),

            },
        )
    }
}
