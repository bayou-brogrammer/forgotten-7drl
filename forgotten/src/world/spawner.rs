use crate::prelude::*;
use gridbugs::{
    coord_2d::Coord,
    entity_table::{entity_data, Entity},
    visible_area_detection::{vision_distance, Light, Rational},
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
                tile: Tile::Player,
                player: Player::new(),
                hp: HitPoints::new_full(100),
                vision: vision_distance::Circle::new(200),
                light: Light {
                    colour: Rgb24::new_grey(200),
                    vision_distance: vision_distance::Circle::new_squared(200),
                    diminish: Rational {numerator: 1, denominator: 8},
                },
            },
        )
    }

    pub fn spawn_wall(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Feature),
            entity_data! {
                tile: Tile::Wall,
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

    pub fn spawn_floor(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Floor),
            entity_data! {
                tile: Tile::Floor,
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

    pub fn spawn_water(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Floor),
            entity_data! {
                tile: Tile::Water,
            },
        );
    }

    pub fn spawn_minibot(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                damage: 1,
                armour: Armour::new(1),
                hp: HitPoints::new_full(3),
                tile: Tile::Npc(NpcType::MiniBot),
                npc: Npc { disposition: Disposition::Hostile, npc_type: NpcType::MiniBot },
            },
        )
    }

    pub fn spawn_secbot(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                damage: 2,
                armour: Armour::new(3),
                hp: HitPoints::new_full(5),
                tile: Tile::Npc(NpcType::SecBot),
                npc: Npc { disposition: Disposition::Hostile, npc_type: NpcType::SecBot },
            },
        )
    }

    pub fn spawn_robocop(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                damage: 3,
                armour: Armour::new(5),
                hp: HitPoints::new_full(10),
                tile: Tile::Npc(NpcType::RoboCop),
                npc: Npc { disposition: Disposition::Hostile, npc_type: NpcType::RoboCop },
            },
        )
    }

    pub fn spawn_doombot(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                damage: 5,
                armour: Armour::new(10),
                hp: HitPoints::new_full(30),
                tile: Tile::Npc(NpcType::DoomBot),
                npc: Npc { disposition: Disposition::Hostile, npc_type: NpcType::DoomBot },
            },
        )
    }

    pub fn spawn_laser(&mut self, start: Coord, target: Coord) {
        println!("Laser fired from {:?} to {:?}", start, target);

        let entity = self.entity_allocator.alloc();
        self.spatial_table.update(entity, Location { coord: start, layer: None }).unwrap();

        self.components.insert_entity_data(
            entity,
            entity_data!(realtime: (), on_collision: OnCollision::Remove, tile: Tile::Laser,),
        );
    }
}
