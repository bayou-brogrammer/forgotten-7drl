use super::explosion;
use crate::{prelude::*, world::realtime};
use gridbugs::{
    coord_2d::Coord,
    entity_table::{entity_data, Entity},
    visible_area_detection::{vision_distance, Light, Rational},
};
use vector::Radians;

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

    pub fn insert_entity_data(&mut self, location: Location, entity_data: EntityData) -> Entity {
        let entity = self.entity_allocator.alloc();
        self.spatial_table.update(entity, location).unwrap();
        self.components.insert_entity_data(entity, entity_data);
        entity
    }

    // Terrain
    pub fn spawn_light(&mut self, coord: Coord, colour: Rgb24) {
        let entity = self.entity_allocator.alloc();
        self.spatial_table.update(entity, Location { coord, layer: None }).unwrap();

        self.components.light.insert(
            entity,
            Light {
                colour,
                vision_distance: vision_distance::Circle::new_squared(200),
                diminish: Rational { numerator: 1, denominator: 10 },
            },
        );
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

    pub fn spawn_stairs(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Feature),
            entity_data! {
                tile: Tile::Stairs,
                stairs: (),
            },
        );
    }

    pub fn spawn_reactor(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Feature),
            entity_data! {
                solid: (),
                reactor: (),
                character: (),
                explodes_on_death: (),
                tile: Tile::Reactor,
                armour: Armour::new(0),
                hp: HitPoints::new_full(30)
            },
        );
    }

    // Entities

    pub fn spawn_player(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                character: (),
                tile: Tile::Player,
                player: Player::new(),
                armour: Armour::new(3),
                hp: HitPoints::new_full(25),
                vision: vision_distance::Circle::new(200),
                light: Light {
                    colour: Rgb24::new_grey(200),
                    vision_distance: vision_distance::Circle::new_squared(200),
                    diminish: Rational {numerator: 1, denominator: 8},
                },
            },
        )
    }

    pub fn spawn_minibot(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                damage: 1,
                character: (),
                armour: Armour::new(1),
                hp: HitPoints::new_full(3),
                tile: Tile::Npc(NpcType::MiniBot),
                npc: Npc {
                    drop_chance: 5,
                    disposition: Disposition::Hostile,
                    npc_type: NpcType::MiniBot,
                    move_to: None,
                    weapon: None
                },
            },
        )
    }

    pub fn spawn_secbot(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                damage: 2,
                character: (),
                armour: Armour::new(3),
                hp: HitPoints::new_full(5),
                tile: Tile::Npc(NpcType::SecBot),
                npc: Npc {
                    drop_chance: 15,
                    disposition: Disposition::Hostile,
                    npc_type: NpcType::SecBot ,
                    move_to: None,
                    weapon: None
                },
            },
        )
    }

    pub fn spawn_robocop(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                damage: 2,
                character: (),
                armour: Armour::new(4),
                hp: HitPoints::new_full(10),
                tile: Tile::Npc(NpcType::RoboCop),
                npc: Npc {
                    drop_chance: 40,
                    disposition:Disposition::Hostile,
                    npc_type:NpcType::RoboCop,
                    move_to: None,
                    weapon: None
                },
            },
        )
    }

    pub fn spawn_doombot(&mut self, coord: Coord) -> Entity {
        let weapon_roll = crate::rng::roll_dice(1, 4);
        let weapon = match weapon_roll {
            0 => Some(Weapon::new_railgun()),
            1 => Some(Weapon::new_rifle()),
            2 => {
                if crate::rng::range(0..=100) < 15 {
                    Some(Weapon::new_fiftycal())
                } else {
                    None
                }
            }
            _ => None,
        };

        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                damage: 4,
                character: (),
                explodes_on_death: (),
                armour: Armour::new(6),
                hp: HitPoints::new_full(20),
                tile: Tile::Npc(NpcType::DoomBot),
                npc: Npc {
                    drop_chance: 80,
                    disposition: Disposition::Hostile,
                    npc_type: NpcType::DoomBot,
                    move_to: None,
                    weapon
                },
            },
        )
    }

    // Items
    pub fn spawn_weapon(&mut self, coord: Coord, ranged_weapon: WeaponType) {
        self.spawn_entity(
            (coord, Layer::Item),
            entity_data! {
                tile:  ranged_weapon.tile(),
                item: Item::Weapon(ranged_weapon),
                weapon: ranged_weapon.new_weapon(),
            },
        );
    }

    pub fn spawn_medkit(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Item),
            entity_data! {
                tile: Tile::Medkit,
                item: Item::Medkit,
            },
        );
    }

    pub fn spawn_upgrade(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Feature),
            entity_data! {
                solid: (),
                upgrade: (),
                tile: Tile::Upgrade,
                item: Item::Medkit,
            },
        );
    }

    pub fn spawn_credit(&mut self, coord: Coord, value: u32) {
        let tile = if value == 1 {
            Tile::Credit1
        } else if value == 2 {
            Tile::Credit2
        } else {
            Tile::Credit3
        };

        self.spawn_entity(
            (coord, Layer::Item),
            entity_data! {
                tile,
                item: Item::Credit(value),
            },
        );
    }

    // Effects
    pub fn spawn_flash(&mut self, coord: Coord, colour: Option<Rgb24>) -> Entity {
        let entity = self.entity_allocator.alloc();
        self.spatial_table.update(entity, Location { coord, layer: None }).unwrap();

        self.components.insert_entity_data(
            entity,
            entity_data!(
                realtime: (),
                light: Light {
                    colour: colour.unwrap_or(Rgb24::new_grey(100)),
                    vision_distance: vision_distance::Circle::new_squared(90),
                    diminish: Rational {numerator: 1, denominator: 20},
                },
            ),
        );
        self.realtime_components.fade.insert(entity, FadeState::new(Duration::from_millis(100)));

        entity
    }

    pub fn spawn_explosion_emitter(
        &mut self,
        coord: Coord,
        spec: &explosion::spec::ParticleEmitter,
    ) -> Entity {
        let emitter_entity = self.entity_allocator.alloc();
        self.spatial_table.update(emitter_entity, Location { coord, layer: None }).unwrap();

        self.components.insert_entity_data(
            emitter_entity,
            entity_data!(realtime: (),
                light: Light {
                    colour: Rgb24::new(255, 187, 63),
                    diminish: Rational {numerator: 1, denominator: 100},
                    vision_distance: vision_distance::Circle::new_squared(420),
                }
            ),
        );

        self.realtime_components.fade.insert(emitter_entity, FadeState::new(spec.duration));
        self.realtime_components.particle_emitter.insert(emitter_entity, {
            use crate::world::realtime::particle::spec::*;
            ParticleEmitter {
                emit_particle_every_period: realtime::period_per_frame(spec.num_particles_per_frame),
                fade_out_duration: Some(spec.duration),
                particle: Particle {
                    tile: None,
                    movement: Some(Movement {
                        angle_range: Radians::uniform_range_all(),
                        cardinal_period_range: UniformInclusiveRange {
                            low: spec.min_step,
                            high: spec.max_step,
                        },
                    }),
                    fade_duration: Some(spec.fade_duration),
                    colour_hint: Some(UniformInclusiveRange {
                        low: Rgb24::new(255, 17, 0),
                        high: Rgb24::new(255, 255, 63),
                    }),
                    possible_particle_emitter: Some(Possible {
                        chance: rational::Rational { numerator: 1, denominator: 20 },
                        value: Box::new(ParticleEmitter {
                            emit_particle_every_period: spec.min_step,
                            fade_out_duration: None,
                            particle: Particle {
                                tile: None,
                                movement: Some(Movement {
                                    angle_range: Radians::uniform_range_all(),
                                    cardinal_period_range: UniformInclusiveRange {
                                        low: Duration::from_millis(200),
                                        high: Duration::from_millis(500),
                                    },
                                }),
                                fade_duration: Some(Duration::from_millis(1000)),
                                ..Default::default()
                            },
                        }),
                    }),
                    ..Default::default()
                },
            }
            .build()
        });

        self.realtime_components.light_colour_fade.insert(
            emitter_entity,
            realtime::light_colour_fade::LightColourFadeState {
                fade_state: realtime::fade::FadeState::new(spec.fade_duration),
                from: Rgb24::new(255, 75, 100),
                to: Rgb24::new(0, 0, 0),
            },
        );

        emitter_entity
    }

    pub fn spawn_bullet(&mut self, start: Coord, target: Coord, weapon: &Weapon) {
        let entity = self.entity_allocator.alloc();
        self.spatial_table.update(entity, Location { coord: start, layer: None }).unwrap();

        self.components.insert_entity_data(
            entity,
            entity_data!(
                realtime: (),
                tile: Tile::Bullet,
                blocks_gameplay: Duration::from_millis(100),
                on_collision: weapon.on_collision.unwrap_or_default(),
                collides_with: weapon.collides_with.unwrap_or_default(),
                projectile_damage: ProjectileDamage {
                    pen: weapon.pen,
                    hit_points: weapon.dmg,
                    weapon_name: Some(weapon.name),
                    stun_chance: weapon.stun_percent,
                    push_back: weapon
                        .abilities
                        .iter()
                        .any(|a| *a ==WeaponAbility::KnockBack),
                    life_steal: weapon
                        .abilities
                        .iter()
                        .any(|a| *a == WeaponAbility::LifeSteal),
                },
            ),
        );

        self.realtime_components.movement.insert(
            entity,
            realtime::movement::spec::Movement {
                path: target - start,
                cardinal_step_duration: Duration::from_millis(15),
                repeat: realtime::movement::spec::Repeat::Once,
            }
            .build(),
        );

        let particle_emitter_ = weapon
            .light_colour
            .map_or_else(
                || {
                    use realtime::particle::spec::*;

                    ParticleEmitter {
                        emit_particle_every_period: Duration::from_micros(2000),
                        fade_out_duration: None,
                        particle: Particle {
                            tile: None,
                            movement: Some(Movement {
                                angle_range: Radians::uniform_range_all(),
                                cardinal_period_range: UniformInclusiveRange {
                                    low: Duration::from_millis(200),
                                    high: Duration::from_millis(500),
                                },
                            }),
                            fade_duration: Some(Duration::from_millis(1000)),
                            possible_light: None,
                            ..Default::default()
                        },
                    }
                },
                |light_colour| {
                    use realtime::particle::spec::*;

                    if weapon.bright {
                        ParticleEmitter {
                            emit_particle_every_period: Duration::from_millis(8),
                            fade_out_duration: None,
                            particle: Particle {
                                tile: None,
                                movement: None,
                                fade_duration: Some(Duration::from_millis(200)),
                                possible_light: Some(Possible {
                                    chance: rational::Rational { numerator: 1, denominator: 1 },
                                    value: Light {
                                        colour: light_colour,
                                        vision_distance: vision_distance::Circle::new_squared(50),
                                        diminish: Rational { numerator: 10, denominator: 1 },
                                    },
                                }),
                                ..Default::default()
                            },
                        }
                    } else {
                        ParticleEmitter {
                            emit_particle_every_period: Duration::from_millis(1),
                            fade_out_duration: None,
                            particle: Particle {
                                tile: None,
                                movement: None,
                                fade_duration: Some(Duration::from_millis(100)),
                                possible_light: Some(Possible {
                                    chance: rational::Rational { numerator: 1, denominator: 1 },
                                    value: Light {
                                        colour: light_colour,
                                        vision_distance: vision_distance::Circle::new_squared(7),
                                        diminish: Rational { numerator: 100, denominator: 1 },
                                    },
                                }),
                                ..Default::default()
                            },
                        }
                    }
                },
            )
            .build();

        self.realtime_components.particle_emitter.insert(entity, particle_emitter_);
    }
}
