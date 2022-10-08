use crate::prelude::*;

pub struct GameInstance {
    pub scope: StateScope,
    pub current_music: Option<Music>,
}

impl GameInstance {
    pub fn new<R: Rng>(config: &GameConfig, rng: &mut R) -> (Self, state::Running) {
        let (scope, running) = state::GameState::new_game(config, rng);
        (GameInstance { scope, current_music: None }, running)
    }

    pub fn into_storable(self, running: state::Running) -> GameInstanceStorable {
        let Self { scope, current_music } = self;
        let running_game = running.running_game(scope);
        GameInstanceStorable { running_game, current_music }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
/// Render
//////////////////////////////////////////////////////////////////////////////////////////
impl GameInstance {
    pub fn render(&self, ctx: Ctx, fb: &mut FrameBuffer) {
        let offset = self.scope.player_coord() - (GAME_VIEW_SIZE / 2);
        let ctx = ctx.add_offset(GAME_VIEW_OFFSET);
        crate::render::render_game_with_visibility(&self.scope, offset, GAME_VIEW_SIZE, ctx, fb);
    }
}
