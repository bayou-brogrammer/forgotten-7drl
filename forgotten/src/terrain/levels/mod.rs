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
            Weapon(wpn) => {
                world.spawn_floor(coord);
                world.spawn_weapon(coord, *wpn);
            }
        }
    }

    (player_entity.expect("didn't create player"), empty_coords)
}

pub fn first_floor() -> Terrain {
    const LEVEL: u8 = 0;

    let grid = procgen::generate_from_str(include_str!("first_floor.txt"));
    // let grid = procgen::generate(Size::new_u16(30, 30), 0);
    let mut world = World::new(Size::new_u16(40, 33), LEVEL);

    let agents = ComponentTable::default();
    let (player_entity, _) = spawn_terrain(grid, &mut world, None);

    Terrain { world, player_entity, agents }
}
