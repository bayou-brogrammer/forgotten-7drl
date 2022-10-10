use std::collections::HashSet;

use crate::prelude::*;

mod builders;
mod levels;
mod procgen;

pub use builders::*;
pub use levels::*;
pub use procgen::*;
use rand::seq::SliceRandom;

pub const FINAL_LEVEL: u32 = 5;

/// The output of terrain generation
pub struct Terrain {
    pub world: World,
    pub player_entity: Entity,
    pub agents: ComponentTable<Agent>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TerrainState {
    chainsaw_floors: HashSet<u8>,
    ranged_weapons: Vec<WeaponType>,
}

impl TerrainState {
    pub fn new() -> Self {
        use crate::WeaponType::*;

        let mut rng = crate::rng::RNG.lock();

        let mut ranged_weapons = vec![Railgun, LifeStealer, LifeStealer, Railgun];
        ranged_weapons.shuffle(&mut *rng);

        let mut floors = (1..=5).collect::<Vec<_>>();
        floors.shuffle(&mut *rng);

        let mut chainsaw_floors = HashSet::new();
        for _ in 0..2 {
            chainsaw_floors.insert(floors.pop().unwrap());
        }

        Self { ranged_weapons, chainsaw_floors }
    }
}

pub fn build_station(level: u8) -> Terrain {
    let mut terrain_state = TerrainState::new();

    if level == 0 {
        return first_floor(terrain_state);
    }

    const STATION_SIZE: Size = Size::new_u16(40, 40);

    let grid = procgen::generate(STATION_SIZE);
    let mut agents = ComponentTable::default();
    let mut world = World::new(STATION_SIZE, level);
    let (player_entity, mut empty_coords) = spawn_terrain(grid, &mut world);

    generate_items(level, &mut world, &mut terrain_state, &mut empty_coords);
    generate_npcs(level, &mut world, &mut empty_coords, &mut agents);

    Terrain { world, player_entity, agents }
}
