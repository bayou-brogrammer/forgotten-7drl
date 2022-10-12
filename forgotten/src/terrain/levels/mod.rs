use gridbugs::grid_2d::Grid;

use crate::{prelude::*, terrain::procgen};

pub fn spawn_terrain(
    grid: Grid<LevelCell>,
    world: &mut World,
    player_data: Option<EntityData>,
) -> (Entity, Vec<Coord>) {
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

                if let Some(ref player_data) = player_data {
                    let location = Location { coord, layer: Some(Layer::Character) };
                    player_entity = Some(world.insert_entity_data(location, player_data.clone()));
                } else {
                    player_entity = Some(world.spawn_player(coord));
                    world.spawn_weapon(coord, WeaponType::CattleProd)
                }
            }
            Stairs => {
                world.spawn_floor(coord);
                world.spawn_stairs(coord);
            }
            Light(color) => {
                world.spawn_light(coord, *color);
                world.spawn_floor(coord);
            }
            Reactor => {
                world.spawn_reactor(coord);
            }
        }
    }

    (player_entity.expect("didn't create player"), empty_coords)
}

pub fn first_floor() -> Terrain {
    println!("Generating first floor");
    const LEVEL: u8 = 0;

    let grid = procgen::generate_from_str(include_str!("first_floor.txt"));
    let mut world = World::new(grid.size(), LEVEL);

    let agents = ComponentTable::default();
    let (player_entity, _) = spawn_terrain(grid, &mut world, None);

    Terrain { world, player_entity, agents }
}

pub fn last_floor(terrain_state: &mut TerrainState, player_data: Option<EntityData>) -> Terrain {
    println!("Generating last floor");

    const STATION_SIZE: Size = Size::new_u16(60, 60);

    let grid = procgen::generate(STATION_SIZE, FINAL_LEVEL);
    let mut world = World::new(grid.size(), FINAL_LEVEL);

    let mut agents = ComponentTable::default();
    let (player_entity, mut empty_coords) = spawn_terrain(grid, &mut world, player_data);

    generate_items(FINAL_LEVEL, &mut world, terrain_state, &mut empty_coords);
    generate_npcs(FINAL_LEVEL, &mut world, &mut empty_coords, &mut agents);

    Terrain { world, player_entity, agents }
}
