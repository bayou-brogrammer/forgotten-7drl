use super::*;
use gridbugs::{
    coord_2d::{Coord, Size},
    direction::CardinalDirection,
    grid_2d::Grid,
    line_2d::Direction,
};

// Params for the Conway's Game of Life Cell Automata which will be used to generate caverns
struct GameOfLifeParams {
    survive_min: u8,
    survive_max: u8,
    resurrect_min: u8,
    resurrect_max: u8,
}

// State for the Conway's Game of Life Cell Automata which will be used to generate caverns
struct GameOfLife {
    alive: Grid<bool>,
    next: Grid<bool>,
}

impl GameOfLife {
    // Initialize state to random values
    fn new(size: Size) -> Self {
        let alive = Grid::new_fn(size, |_| crate::rng::gen::<bool>());
        let next = Grid::new_default(size);
        Self { alive, next }
    }

    // Progress the cell automata simulation by one step
    fn step(
        &mut self,
        &GameOfLifeParams {
            survive_min,
            survive_max,
            resurrect_min,
            resurrect_max,
        }: &GameOfLifeParams,
    ) {
        for ((coord, &alive_cell), next_cell) in
            self.alive.enumerate().zip(self.next.iter_mut())
        {
            let n: u8 = Direction::all()
                .map(|direction| {
                    self.alive.get(coord + direction.coord()).cloned().unwrap_or(false) as u8
                })
                .sum();
            *next_cell = (alive_cell && n >= survive_min && n <= survive_max)
                || (!alive_cell && n >= resurrect_min && n <= resurrect_max);
        }

        std::mem::swap(&mut self.alive, &mut self.next);
    }
}

// Generate the starting point for the cavern map by running a cell automata for several steps
fn generate_initial_cavern_map(size: Size) -> Grid<FloorOrWall> {
    const NUM_STEPS: usize = 10;
    let mut game_of_life = GameOfLife::new(size);
    // This choice of params leads to cavernous regions of living cells
    let params = GameOfLifeParams {
        survive_min: 4,
        survive_max: 8,
        resurrect_min: 5,
        resurrect_max: 5,
    };

    for _ in 0..NUM_STEPS {
        game_of_life.step(&params);
    }

    Grid::new_grid_map(game_of_life.alive, |alive| {
        if alive {
            FloorOrWall::Floor
        } else {
            FloorOrWall::Wall
        }
    })
}

// Place walls at every point along the outside of a map
fn surround_map_with_walls(map: &mut Grid<FloorOrWall>) {
    let Coord { x: width, y: height } = map.size().to_coord().unwrap();
    for (Coord { x, y }, cell) in map.enumerate_mut() {
        if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
            *cell = FloorOrWall::Wall;
        }
    }
}

// Remove clumps of wall cells which aren't connected to the edge of the map by walls (replacing
// them with floor cells)
fn remove_disconnected_walls(map: &mut Grid<FloorOrWall>) {
    assert!(
        *map.get_checked(Coord::new(0, 0)) == FloorOrWall::Wall,
        "top-left cell must be wall"
    );
    // Flood-fill all the wall cells starting with the top-left
    let mut walls_to_visit = vec![Coord::new(0, 0)];
    let mut seen = Grid::new_copy(map.size(), false);
    *seen.get_checked_mut(Coord::new(0, 0)) = true;
    while let Some(coord) = walls_to_visit.pop() {
        for neighbour_coord in CardinalDirection::all().map(|d| coord + d.coord()) {
            if let Some(FloorOrWall::Wall) = map.get(neighbour_coord) {
                let seen = seen.get_checked_mut(neighbour_coord);
                if !*seen {
                    *seen = true;
                    walls_to_visit.push(neighbour_coord);
                }
            }
        }
    }
    // Update the map, marking all unseen cells as floor
    for (cell_mut, &seen) in map.iter_mut().zip(seen.iter()) {
        if !seen {
            *cell_mut = FloorOrWall::Floor;
        }
    }
}

pub fn generate_cave_map(size: Size) -> Grid<FloorOrWall> {
    let mut map = generate_initial_cavern_map(size);
    surround_map_with_walls(&mut map);
    remove_disconnected_walls(&mut map);
    map
}
