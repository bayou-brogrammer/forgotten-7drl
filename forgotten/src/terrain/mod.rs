use crate::prelude::*;

mod visibility;
pub use visibility::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Terrain {
    pub world: World,
    pub player_entity: Entity,
}

impl Default for Terrain {
    fn default() -> Self {
        Self::new()
    }
}

impl Terrain {
    pub fn new() -> Self {
        let s = include_str!("./terrain.txt");
        let rows = s.split('\n').filter(|s| !s.is_empty()).collect::<Vec<_>>();
        let size = Size::new_u16(rows[0].len() as u16, rows.len() as u16);

        let mut world = World::new(size);
        let mut player_data = Some(World::make_player());
        let mut player_entity = None;
        for (y, row) in rows.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch.is_control() {
                    continue;
                }
                let coord = Coord::new(x as i32, y as i32);
                match ch {
                    '.' => {
                        world.spawn_floor(coord);
                    }
                    'R' => {
                        world.spawn_floor(coord);
                        world.spawn_light(coord, Rgb24::new(255, 0, 0));
                    }
                    'G' => {
                        world.spawn_floor(coord);
                        world.spawn_light(coord, Rgb24::new(0, 255, 0));
                    }
                    '#' => {
                        world.spawn_wall(coord);
                    }
                    '@' => {
                        world.spawn_floor(coord);
                        let location = Location { coord, layer: Some(Layer::Character) };
                        player_entity = Some(world.insert_entity_data(location, player_data.take().unwrap()));
                    }

                    other => panic!("unexpected char {}", other),
                }
            }
        }

        let player_entity = player_entity.expect("didn't create player");
        Terrain { world, player_entity }
    }
}
