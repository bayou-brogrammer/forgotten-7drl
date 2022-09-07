mod flow;
mod player;
mod sound;
mod terrain;
mod world;

pub mod witness;

pub mod prelude {
    pub use gridbugs::grid_search_cardinal::CardinalDirection;
    pub use gridbugs::rgb_int::Rgb24;
    pub use gridbugs::shadowcast::Context as ShadowcastContext;
    pub use gridbugs::spatial_table::{Coord, Entity, Size};

    pub use rand::{Rng, SeedableRng};
    pub use rand_isaac::Isaac64Rng;
    pub use serde::{Deserialize, Serialize};
    pub use std::time::Duration;

    pub use macros::*;

    pub use crate::flow::*;
    pub use crate::player::*;
    pub use crate::sound::*;
    pub use crate::terrain::*;
    pub use crate::witness::*;
    pub use crate::world::*;
    pub use crate::{Game, GameConfig};
}
pub use prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct GameConfig {
    pub debug: bool,
    pub omniscient: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    rng: Isaac64Rng,

    world: World,
    player_entity: Entity,
    visibility_grid: VisibilityGrid,
    shadowcast_context: ShadowcastContext<u8>,

    // Duration
    since_last_frame: Duration,
}

impl Game {
    pub fn new<R: Rng>(config: &GameConfig, base_rng: &mut R) -> Self {
        let mut rng = Isaac64Rng::from_rng(base_rng).unwrap();

        let Terrain { world, player_entity } = Terrain::new();
        let visibility_grid = VisibilityGrid::new(world.size());
        let shadowcast_context = ShadowcastContext::default();

        let mut game = Self {
            rng,
            world,
            player_entity,
            visibility_grid,
            shadowcast_context,
            since_last_frame: Duration::from_millis(0),
        };
        game.update_visibility(config);
        game
    }

    pub fn player_walk(
        &mut self,
        direction: CardinalDirection,
        config: &GameConfig,
    ) -> Result<Option<ControlFlow>, ActionError> {
        let flow = self.world.character_walk_in_direction(&mut self.rng, self.player_entity, direction)?;
        self.update_visibility(config);
        Ok(flow)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
/// Visibility
//////////////////////////////////////////////////////////////////////////////////////////

impl Game {
    pub fn visibility_grid(&self) -> &VisibilityGrid {
        &self.visibility_grid
    }

    fn update_visibility(&mut self, config: &GameConfig) {
        if let Some(player_coord) = self.world.entity_coord(self.player_entity) {
            let mut map = None;
            if let Some(layers) = self.world.spatial_table.layers_at(player_coord) {
                if let Some(feature) = layers.feature {
                    if self.world.components.map.contains(feature) {
                        map = Some(true)
                    }
                }
            }

            self.visibility_grid.update(
                player_coord,
                &self.world,
                &mut self.shadowcast_context,
                map.unwrap_or(config.omniscient),
            );
        }
    }
}
