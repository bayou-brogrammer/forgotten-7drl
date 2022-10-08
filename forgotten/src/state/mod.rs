use crate::prelude::*;

mod prompt;
mod running;

pub use prompt::*;
pub use running::*;

/// The `Witness` type defined in this module is intended as the sole means of mutating the game
/// state. Depending on the current state of the game, different types of mutation may be valid or
/// invalid. For example, if the game is in a state where the user is expected to choose an option
/// from a menu, such as an ability to use, it is invalid for the game to receive an update which
/// moves the player character. One solution to this problem would be to have all mutating methods
/// of the `Game` type take ownership of `self` and return an `enum` encoding the different types
/// of interaction the game could expect. This would be inconvenient to clients of this code as
/// it will prevent mutating the game state through a `mut ref`. The `Witness` type encodes the
/// currently-expected interaction externally to the game's state itself, and exposes methods that
/// mutate the game state through a `mut ref`, and take ownership of, and return witness values to
/// model changes in the currently-expected updates and prevent invalid updates with the type
/// system.

/// A private unit type which prevents witnesses being minted other than in the approved ways.
/// Importantly, this type is not `Clone` or `Copy`, and neither are any witness types, similarly
/// to control the construction of witnesses.
#[derive(Debug)]
pub(crate) struct Private;

#[derive(Debug)]
pub enum GameState {
    Win,
    GameOver,
    Prompt(Prompt),
    Running(Running),
}

impl GameState {
    pub fn new_game<R: Rng>(config: &GameConfig, base_rng: &mut R) -> (StateScope, Running) {
        let g = crate::Game::new(config, base_rng);
        (StateScope(g), Running(Private))
    }
}

#[derive(Serialize, Deserialize)]
pub struct StateScope(pub crate::Game);

mod game_interface {
    use super::StateScope;
    use crate::{Fetch, ToRenderEntity, VisibleCellData};
    use gridbugs::{coord_2d::Coord, visible_area_detection::VisibilityGrid};

    impl StateScope {
        //////////////////////////////
        // Spatial queries
        //////////////////////////////
        pub fn player_coord(&self) -> Coord {
            self.0.player_coord()
        }

        pub fn is_wall_known_at(&self, coord: Coord) -> bool {
            self.0.is_wall_known_at(coord)
        }

        pub fn to_render_entities(&self) -> impl '_ + Iterator<Item = ToRenderEntity> {
            self.0.world.to_render_entities()
        }

        //////////////////////////////
        // Visibility
        //////////////////////////////
        pub fn visibility_grid(&self) -> Fetch<VisibilityGrid<VisibleCellData>> {
            self.0.world.fetch::<VisibilityGrid<VisibleCellData>>()
        }
    }
}