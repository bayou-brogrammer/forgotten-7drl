pub mod color;
pub mod render;
pub mod storage;

mod action;
mod audio;
mod controls;
mod game_instance;
mod game_loop;
mod instances;
mod rng;

mod prelude {
    pub use gridbugs::chargrid::prelude::*;
    pub use gridbugs::rgb_int::Rgb24;
    pub use gridbugs::spatial_table::{Coord, Size};
    pub use gridbugs::{chargrid::control_flow::*, grid_search_cardinal::CardinalDirection};
    pub use rand::Rng;
    pub use serde::{Deserialize, Serialize};

    pub use forgotten_game::prelude::*;
    pub use forgotten_game::witness;

    pub use crate::action::*;
    pub use crate::audio::*;
    pub use crate::controls::*;
    pub use crate::game_instance::*;
    pub use crate::game_loop::*;
    pub use crate::render::*;
    pub use crate::rng::*;
    pub use crate::storage::*;
    pub use crate::AppConfig;
    pub use crate::{color, storage};

    // pub const GAME_VIEW_SIZE: Size = Size::new_u16(26, 18);
    pub const GAME_VIEW_SIZE: Size = Size::new_u16(20, 30);
    pub const GAME_VIEW_OFFSET: Coord = Coord::new(0, 0);
    pub const LAUNCHER_TITLE: &str = "F.o.r.g.o.t.t.e.n";
}
pub use prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AppConfig {
    won: bool,
    sfx_volume: f32,
    first_run: bool,
    music_volume: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { music_volume: 0.2, sfx_volume: 0.5, won: false, first_run: true }
    }
}

pub struct AppState {
    game_loop_data: game_loop::GameLoopData,
}

impl_new!(AppState, game_loop_data: game_loop::GameLoopData);

pub struct AppArgs {
    pub new_game: bool,
    pub omniscient: bool,
    pub storage: AppStorage,
    pub audio_player: AppAudioPlayer,
    pub initial_rng_seed: InitialRngSeed,
}

pub fn run_app(AppArgs { storage, initial_rng_seed, omniscient, new_game, audio_player }: AppArgs) -> App {
    let config = GameConfig { omniscient, debug: false };

    let (game_loop_data, initial_state) =
        game_loop::GameLoopData::new(config, storage, initial_rng_seed, audio_player, new_game);

    game_loop::game_loop_component(initial_state)
        .map(|_| app::Exit)
        .with_state(game_loop_data)
        .clear_each_frame()
        .exit_on_close()
}
