mod ai;
mod behavior;
mod flow;
mod player;
mod prompt;
mod sound;
mod terrain;
mod visibility;
mod world;

pub mod event;
pub mod log;
pub mod rng;
pub mod state;

pub mod prelude {
    pub use gridbugs::entity_table::*;
    pub use gridbugs::grid_search_cardinal::CardinalDirection;
    pub use gridbugs::line_2d::Direction;
    pub use gridbugs::rgb_int::*;
    pub use gridbugs::shadowcast::Context as ShadowcastContext;
    pub use gridbugs::spatial_table::{Coord, Entity, Size};

    pub use rand::{Rng, SeedableRng};
    pub use rand_isaac::Isaac64Rng;
    pub use serde::{Deserialize, Serialize};
    pub use std::time::Duration;

    pub use macros::*;

    pub use crate::ai::*;
    pub use crate::behavior::*;
    pub use crate::event::*;
    pub use crate::flow::*;
    pub use crate::log::*;
    pub use crate::player::*;
    pub use crate::prompt::*;
    pub use crate::sound::*;
    pub use crate::state::*;
    pub use crate::terrain::*;
    pub use crate::visibility::*;
    pub use crate::world::*;

    pub use crate::{Game, GameConfig};
}

use gridbugs::visible_area_detection::VisibilityGrid;
pub use prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GameConfig {
    pub debug: bool,
    pub omniscient: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum TurnState {
    PlayerTurn,
    EnemyTurn,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    start: bool,
    pub config: GameConfig,
    pub world: World,
    pub player_entity: Entity,

    pub turn_state: TurnState,
    pub agents: ComponentTable<Agent>,
    pub behavior_context: BehaviourContext,
    pub visibility_grid: VisibilityGrid<VisibleCellData>,

    // Duration
    since_last_frame: Duration,
    animation_context: AnimationContext,
}

impl Game {
    pub fn new<R: Rng>(config: &GameConfig, base_rng: &mut R) -> Self {
        crate::rng::reseed_from_rng(base_rng);

        let Terrain { player_entity, world, agents } = Terrain::generate(Size::new(80, 60), 1);
        let visibility_grid = VisibilityGrid::new(world.size());
        let behavior_context = BehaviourContext::new(world.size());

        let mut game = Self {
            world,
            agents,
            start: true,
            player_entity,
            config: *config,
            visibility_grid,
            behavior_context,
            turn_state: TurnState::PlayerTurn,
            since_last_frame: Duration::from_millis(0),
            animation_context: AnimationContext::default(),
        };
        game.update_visibility();
        game.prime_npcs();
        game
    }

    pub fn is_game_over(&self) -> bool {
        self.world.components.dead.get(self.player_entity).is_some()
    }

    pub fn is_won(&self) -> bool {
        false
    }

    pub fn current_level(&self) -> u32 {
        self.world.level
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
/// Spatial
//////////////////////////////////////////////////////////////////////////////////////////

impl Game {
    pub fn entity_coord(&self, entity: Entity) -> Option<Coord> {
        self.world.spatial_table.coord_of(entity)
    }

    /// Returns true iff a wall has been seen by the player at the given coord
    pub fn is_wall_known_at(&self, coord: Coord) -> bool {
        if let Some(data) = self.visibility_grid.get_data(coord) {
            data.tiles.feature.map(|tile| tile.is_wall()).unwrap_or(false)
        } else {
            false
        }
    }
}
