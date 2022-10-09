use std::collections::HashSet;

use self::npc::generate_npcs;
use crate::{Agent, World};
use gridbugs::{
    coord_2d::{Axis, Coord, Size},
    direction::CardinalDirection,
    entity_table::{ComponentTable, Entity},
    grid_2d::Grid,
    line_2d::Direction,
    perlin2::Perlin2,
};
use rand::Rng;

mod cave;
mod npc;
mod rooms;
use rooms::*;

pub const FINAL_LEVEL: u32 = 5;

fn print_map(grid: &Grid<LevelCell>) {
    for row in grid.rows() {
        for &cell in row {
            use LevelCell::*;
            let ch = match cell {
                Wall => '#',
                Floor => '.',
                Door => '+',
                CaveFloor => ',',
                CaveWall => '%',
                Grass => '"',
            };
            print!("{}", ch);
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_room_map(grid: &Grid<RoomsAndCorridorsCell>) {
    for row in grid.rows() {
        for &cell in row {
            let ch = match cell {
                RoomsAndCorridorsCell::Floor => '.',
                RoomsAndCorridorsCell::Wall => '#',
                RoomsAndCorridorsCell::Door => '+',
            };
            print!("{}", ch);
        }
        println!();
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FloorOrWall {
    Floor,
    Wall,
}

// A cell of the game world
#[derive(Clone, Copy, PartialEq, Eq)]
enum LevelCell {
    Floor,
    Wall,
    Door,
    CaveFloor,
    CaveWall,
    Grass,
}

impl LevelCell {
    fn is_wall(&self) -> bool {
        matches!(self, Self::Wall | Self::CaveWall)
    }

    fn is_floor(&self) -> bool {
        matches!(self, Self::Floor | Self::CaveFloor)
    }
}

fn is_surrounded_by_walls(map: &Grid<RoomsAndCorridorsCell>, coord: Coord) -> bool {
    Direction::all()
        .filter_map(|direction| map.get(coord + direction.coord()))
        .all(|&cell| cell == RoomsAndCorridorsCell::Wall)
}

// Combines a map of rooms and corridors with a cave map to produce a hybrid of the two
fn combine_rooms_and_corridors_level_with_cave(
    rooms_and_corridors_level_map: &Grid<RoomsAndCorridorsCell>,
    cave_map: &Grid<FloorOrWall>,
) -> Grid<LevelCell> {
    Grid::new_fn(cave_map.size(), |coord| match cave_map.get_checked(coord) {
        FloorOrWall::Floor => LevelCell::CaveFloor,
        FloorOrWall::Wall => match rooms_and_corridors_level_map.get_checked(coord) {
            RoomsAndCorridorsCell::Floor => LevelCell::Floor,
            RoomsAndCorridorsCell::Door => LevelCell::Door,
            RoomsAndCorridorsCell::Wall => {
                if is_surrounded_by_walls(rooms_and_corridors_level_map, coord) {
                    LevelCell::CaveWall
                } else {
                    LevelCell::Wall
                }
            }
        },
    })
}

fn remove_unreachable_floor(map: &mut Grid<LevelCell>, water_map: &mut Grid<bool>, player_spawn: Coord) {
    let mut seen = Grid::new_copy(map.size(), false);
    *seen.get_checked_mut(player_spawn) = true;
    let mut to_visit = vec![player_spawn];
    while let Some(current) = to_visit.pop() {
        for direction in CardinalDirection::all() {
            let neighbour_coord = current + direction.coord();
            if let Some(neighbour_cell) = map.get(neighbour_coord) {
                let water_cell = *water_map.get_checked(neighbour_coord);
                if !neighbour_cell.is_wall() || water_cell {
                    let seen_cell = seen.get_checked_mut(neighbour_coord);
                    if !*seen_cell {
                        to_visit.push(neighbour_coord);
                    }
                    *seen_cell = true;
                }
            }
        }
    }

    for ((&seen_cell, map_cell), water_cell) in seen.iter().zip(map.iter_mut()).zip(water_map.iter_mut()) {
        if !seen_cell {
            *water_cell = false;
            if *map_cell == LevelCell::CaveFloor {
                *map_cell = LevelCell::CaveWall;
            }
        }
    }
}

fn is_valid_door_position_axis(map: &Grid<LevelCell>, coord: Coord, axis: Axis) -> bool {
    let axis_delta = Coord::new_axis(1, 0, axis);
    let other_axis_delta = Coord::new_axis(0, 1, axis);
    let floor_in_axis =
        map.get_checked(coord + axis_delta).is_floor() && map.get_checked(coord - axis_delta).is_floor();
    let wall_in_other_axis = map.get_checked(coord + other_axis_delta).is_wall()
        && map.get_checked(coord - other_axis_delta).is_wall();
    floor_in_axis && wall_in_other_axis
}

fn is_valid_door_position(map: &Grid<LevelCell>, coord: Coord) -> bool {
    is_valid_door_position_axis(map, coord, Axis::X) || is_valid_door_position_axis(map, coord, Axis::Y)
}

// Updates a map, replacing all door cells which aren't in valid positions with floor cells. A door
// can be in an invalid position due to the effect of combining a room and corridor map with a cave
// map.
fn remove_invalid_doors(map: &mut Grid<LevelCell>) {
    let to_remove = map
        .enumerate()
        .filter_map(|(coord, cell)| {
            if *cell == LevelCell::Door && !is_valid_door_position(map, coord) {
                Some(coord)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    for coord in to_remove {
        *map.get_checked_mut(coord) = LevelCell::Floor;
    }
}

fn add_grass(map: &mut Grid<LevelCell>) {
    let mut rng = crate::rng::RNG.lock();
    let perlin = Perlin2::new(&mut *rng);
    let zoom = 10.;
    for (Coord { x, y }, cell) in map.enumerate_mut() {
        if *cell == LevelCell::CaveFloor {
            let x = x as f64 / zoom;
            let y = y as f64 / zoom;
            let noise = perlin.noise((x, y));
            if noise > 0. && rng.gen::<f64>() > noise {
                *cell = LevelCell::Grass;
            }
        }
    }

    std::mem::drop(rng);
}

// Returns a grid of booleans, where a true value indicates that grass can spawn at that location.
// The grid is populated using perlin noise.
fn make_water_map(size: Size) -> Grid<bool> {
    let mut rng = crate::rng::RNG.lock();
    let perlin = Perlin2::new(&mut *rng);
    let zoom = 7.;
    let mut map = Grid::new_fn(size, |Coord { x, y }| {
        let x = x as f64 / zoom;
        let y = y as f64 / zoom;
        let noise = perlin.noise01((x, y));
        noise > 0.65
    });

    let mut to_visit = map
        .edge_enumerate()
        .filter_map(|(coord, cell)| if *cell { Some(coord) } else { None })
        .collect::<Vec<_>>();

    let mut seen = to_visit.iter().cloned().collect::<HashSet<_>>();
    while let Some(coord) = to_visit.pop() {
        for direction in CardinalDirection::all() {
            let neighbour_coord = coord + direction.coord();
            if let Some(true) = map.get(neighbour_coord) {
                if seen.insert(neighbour_coord) {
                    to_visit.push(neighbour_coord);
                }
            }
        }
    }

    for coord in seen {
        *map.get_checked_mut(coord) = false;
    }
    map
}

/// The output of terrain generation
pub struct Terrain {
    pub world: World,
    pub player_entity: Entity,
    pub agents: ComponentTable<Agent>,
}

impl Terrain {
    pub fn generate(size: Size, level: u32) -> Self {
        let mut world = World::new(size, level);

        let RoomsAndCorridorsLevel { map: rooms_and_corridors_map, player_spawn } =
            RoomsAndCorridorsLevel::generate(size);
        let cave_map = cave::generate_cave_map(size);
        let mut combined_map =
            combine_rooms_and_corridors_level_with_cave(&rooms_and_corridors_map, &cave_map);

        let mut water_map = make_water_map(size);
        remove_unreachable_floor(&mut combined_map, &mut water_map, player_spawn);
        remove_invalid_doors(&mut combined_map);
        add_grass(&mut combined_map);
        print_map(&combined_map);

        let player_entity = world.spawn_player(player_spawn);
        let mut npc_candidates = Vec::new();
        for (coord, &cell) in combined_map.enumerate() {
            use LevelCell::*;
            if *water_map.get_checked(coord) {
                match cell {
                    Floor | Door => world.spawn_water(coord),
                    Wall => {
                        if crate::rng::range(0..=100) < 75 {
                            world.spawn_wall(coord)
                        } else {
                            world.spawn_water(coord);
                        }
                    }
                    CaveFloor | CaveWall => {
                        world.spawn_water(coord);
                    }
                    Grass => {
                        world.spawn_grass(coord);
                    }
                }
            } else {
                match cell {
                    Floor => {
                        world.spawn_floor(coord);
                        npc_candidates.push(coord);
                    }
                    CaveFloor => {
                        world.spawn_cave_floor(coord);
                        npc_candidates.push(coord);
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
                }
            }
        }

        npc_candidates.retain(|c| *c != player_spawn);
        let mut agents = ComponentTable::default();
        generate_npcs(&mut world, level, &mut npc_candidates, &mut agents);

        Self { world, player_entity, agents }
    }
}
