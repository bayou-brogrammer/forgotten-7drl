use gridbugs::grid_2d::Grid;

use crate::{prelude::*, terrain::procgen};

pub fn spawn_terrain(grid: Grid<LevelCell>, world: &mut World) -> (Entity, Vec<Coord>) {
    let mut player_entity = None;
    let mut empty_coords = Vec::new();
    for (coord, cell) in grid.enumerate() {
        use LevelCell::*;

        match cell {
            Floor => {
                world.spawn_floor(coord);
                empty_coords.push(coord);
            }
            CaveFloor => {
                world.spawn_cave_floor(coord);
                empty_coords.push(coord);
            }
            Wall => world.spawn_wall(coord),
            CaveWall => world.spawn_cave_wall(coord),
            Door => {
                world.spawn_floor(coord);
                world.spawn_door(coord);
            }
            Grass => {
                world.spawn_cave_floor(coord);
                world.spawn_grass(coord);
            }
            Water => {
                world.spawn_water(coord);
            }
            PlayerSpawn => {
                world.spawn_floor(coord);
                player_entity = Some(world.spawn_player(coord));
                world.spawn_weapon(coord, WeaponType::LifeStealer)
            }
        }
    }

    (player_entity.expect("didn't create player"), empty_coords)
}

pub fn first_floor(mut terrain_state: TerrainState) -> Terrain {
    const LEVEL: u8 = 0;
    const SIZE: Size = Size::new_u16(10, 10);

    let grid = procgen::generate(SIZE);
    let mut world = World::new(SIZE, 0);

    let mut agents = ComponentTable::default();
    let (player_entity, mut empty_coords) = spawn_terrain(grid, &mut world);

    generate_items(LEVEL, &mut world, &mut terrain_state, &mut empty_coords);
    generate_npcs(LEVEL, &mut world, &mut empty_coords, &mut agents);

    Terrain { world, player_entity, agents }
}
