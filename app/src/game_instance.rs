use gridbugs::chargrid::text::StyledString;

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
        self.render_hud(ctx, fb);
        self.render_message_log(ctx, fb);

        let offset = self.scope.player_coord() - (GAME_VIEW_SIZE / 2);
        let ctx = ctx.add_offset(GAME_VIEW_OFFSET);
        crate::render::render_game_with_visibility(&self.scope, offset, GAME_VIEW_SIZE, ctx, fb);
    }

    pub fn render_message_log(&self, ctx: Ctx, fb: &mut FrameBuffer) {
        crate::ui::render_message_log(
            &self.scope,
            ctx.add_offset(Coord { x: 0, y: GAME_VIEW_SIZE.y() as i32 + 2 }),
            fb,
        );
    }

    pub fn render_hud(&self, ctx: Ctx, fb: &mut FrameBuffer) {
        crate::render::ui::render_hud(&self.scope, ctx.add_xy(GAME_UI_OFFSET.x, GAME_UI_OFFSET.y), fb);
    }

    pub fn floor_text(&self) -> StyledString {
        let current_floor = self.scope.current_level();
        let final_floor = forgotten_game::FINAL_LEVEL;

        if current_floor == 0 {
            StyledString {
                style: Style::new().with_foreground(Rgba32::new_grey(255)).with_bold(true),
                string: format!("Gotta get to the fuel bay on the {}th floor...", final_floor),
            }
        } else {
            StyledString {
                style: Style::new().with_foreground(Rgba32::new_grey(255)).with_bold(true),
                string: format!("Floor {}/{}", current_floor, final_floor),
            }
        }
    }
}
