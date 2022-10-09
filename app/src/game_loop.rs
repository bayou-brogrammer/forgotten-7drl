use std::time::Duration;

use super::{game_instance::GameInstance, prelude::*};
use gridbugs::chargrid::text::StyledString;
use rand_xorshift::XorShiftRng;

/// An interactive, renderable process yielding a value of type `T`
pub type AppCF<T> = CF<Option<T>, GameLoopData>;
pub type State = GameLoopData;

#[derive(Debug)]
pub enum GameLoopState {
    MainMenu,
    Paused(state::Running),
    Playing(state::GameState),
}

pub struct GameLoopData {
    pub config: AppConfig,
    pub controls: Controls,
    pub storage: AppStorage,
    pub cursor: Option<Coord>,

    pub audio_state: AudioState,
    pub game_config: GameConfig,
    pub instance: Option<GameInstance>,
    pub context_message: Option<StyledString>,
    pub examine_message: Option<StyledString>,

    // rng
    pub duration: Duration,
    pub effect_rng: XorShiftRng,
    pub rng_seed_source: RngSeedSource,
}

fn new_game(rng_seed_source: &mut RngSeedSource, game_config: &GameConfig) -> (GameInstance, state::Running) {
    let mut rng = Isaac64Rng::seed_from_u64(rng_seed_source.next_seed());
    GameInstance::new(game_config, &mut rng)
}

impl GameLoopData {
    pub fn new(
        game_config: GameConfig,
        mut storage: AppStorage,
        initial_rng_seed: InitialRngSeed,
        audio_player: AppAudioPlayer,
        force_new_game: bool,
    ) -> (Self, GameLoopState) {
        let mut rng_seed_source = RngSeedSource::new(initial_rng_seed);
        let (instance, state) = match storage.load_game() {
            Some(instance) => {
                let (instance, running) = instance.into_game_instance();
                (Some(instance), GameLoopState::Playing(running.into_witness()))
            }
            None => {
                if force_new_game {
                    let (instance, running) = new_game(&mut rng_seed_source, &game_config);
                    (Some(instance), GameLoopState::Playing(running.into_witness()))
                } else {
                    (None, GameLoopState::MainMenu)
                }
            }
        };

        let controls = if let Some(controls) = storage.load_controls() {
            controls
        } else {
            let controls = Controls::default();
            storage.save_controls(&controls);
            controls
        };

        let mut audio_state = AudioState::new(audio_player);
        let config = storage.load_config().unwrap_or_default();

        if let Some(instance) = instance.as_ref() {
            if let Some(music) = instance.current_music {
                audio_state.loop_music(game_music_to_audio(music), config.music_volume);
            }
        } else {
            audio_state.loop_music(Audio::Menu, config.music_volume);
        }

        let game_loop_data = Self {
            audio_state,
            config,
            storage,
            controls,
            instance,
            game_config,
            cursor: None,
            rng_seed_source,
            context_message: None,
            examine_message: None,
            duration: Duration::from_millis(0),
            effect_rng: XorShiftRng::from_entropy(),
        };

        (game_loop_data, state)
    }

    pub(crate) fn new_game(&mut self) -> state::Running {
        let (instance, running) = new_game(&mut self.rng_seed_source, &self.game_config);
        self.instance = Some(instance);
        running
    }

    pub fn scope(&self) -> &StateScope {
        &self.instance.as_ref().unwrap().scope
    }

    pub fn handle_game_events(&mut self) {
        let instance = self.instance.as_mut().unwrap();
        for event in instance.scope.events() {
            match event {
                ExternalEvent::LoopMusic(music) => {
                    instance.current_music = Some(music);
                    self.audio_state.loop_music(game_music_to_audio(music), self.config.music_volume);
                }
                ExternalEvent::SoundEffect(sound_effect) => {
                    self.audio_state.play_once(Audio::SoundEffect(sound_effect), self.config.sfx_volume);
                }
                ExternalEvent::Explosion(_coord) => {
                    // self.screen_shake = Some(ScreenShake {
                    //     direction: self.effect_rng.gen(),
                    //     remaining: Duration::from_millis(100),
                    // });
                    // self.audio_state.play_once(Audio::Explosion, self.config.sfx_volume);
                }
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
/// Storage
//////////////////////////////////////////////////////////////////////////////////////////

impl GameLoopData {
    pub fn save_config(&mut self) {
        self.storage.save_config(&self.config);
    }

    pub fn save_instance(&mut self, running: state::Running) -> state::Running {
        let instance = self.instance.take().unwrap().into_storable(running);
        self.storage.save_game(&instance);
        let (instance, running) = instance.into_game_instance();
        self.instance = Some(instance);
        running
    }

    pub fn clear_saved_game(&mut self) {
        self.storage.clear_game();
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
/// Rendering
//////////////////////////////////////////////////////////////////////////////////////////

impl GameLoopData {
    pub fn render(&self, cursor_colour: Rgba32, ctx: Ctx, fb: &mut FrameBuffer) {
        let instance = self.instance.as_ref().unwrap();
        instance.render(ctx, fb);

        if let Some(cursor) = self.cursor {
            if cursor.is_valid(GAME_VIEW_SIZE + Size::new_u16(1, 1)) {
                let screen_cursor = GAME_VIEW_OFFSET + (cursor * 3);
                for offset in Size::new_u16(3, 3).coord_iter_row_major() {
                    fb.set_cell_relative_to_ctx(
                        ctx,
                        screen_cursor + offset,
                        10,
                        RenderCell::BLANK.with_background(cursor_colour),
                    );
                }
            }
        }

        self.render_text(ctx, fb);
    }

    pub fn render_stars(&self, ctx: Ctx, fb: &mut FrameBuffer) {}

    fn render_text(&self, ctx: Ctx, fb: &mut FrameBuffer) {
        let instance = self.instance.as_ref().unwrap();
        if let Some(context_message) = self.context_message.as_ref() {
            context_message.render(&(), ctx.add_y(1), fb);
        }

        if let Some(top_text) = self.examine_message.as_ref() {
            top_text.clone().wrap_word().render(&(), ctx, fb);
        } else {
            instance.floor_text().render(&(), ctx, fb);
        }
    }

    #[allow(clippy::collapsible_match)]
    pub fn examine_mouse(&mut self, event: Event) {
        if let Event::Input(Input::Mouse(mouse_input)) = event {
            if let MouseInput::MouseMove { button: _, coord } = mouse_input {
                let cursor = (coord - GAME_VIEW_OFFSET) / 3;
                if cursor.is_valid(GAME_VIEW_SIZE) {
                    self.cursor = Some(cursor);
                } else {
                    self.cursor = None;
                }
            }
        }
    }

    pub fn update_examine_text(&mut self) {
        self.examine_message = self.cursor.and_then(|coord| {
            let world_coord = self.scope().player_coord() - (GAME_VIEW_SIZE / 2) + coord;
            examine(self.scope(), world_coord)
        });
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
/// Game Loop
//////////////////////////////////////////////////////////////////////////////////////////

pub fn game_loop_component(initial_state: GameLoopState) -> AppCF<()> {
    use crate::instances::*;
    use GameLoopState::*;

    first_run_prologue().then(|| {
        loop_(initial_state, |state| match state {
            MainMenu => main_menu_loop().map(|main_menu_output| match main_menu_output {
                MainMenuOutput::Quit => LoopControl::Break(()),
                MainMenuOutput::NewGame { new_running } => {
                    LoopControl::Continue(Playing(new_running.into_witness()))
                }
            }),
            Paused(running) => pause_menu_loop(running).map(|pause_output| match pause_output {
                PauseOutput::Quit => LoopControl::Break(()),
                PauseOutput::MainMenu => LoopControl::Continue(MainMenu),
                PauseOutput::ContinueGame { running } => {
                    LoopControl::Continue(Playing(running.into_witness()))
                }
            }),
            Playing(witness) => match witness {
                GameState::Win => todo!(),
                GameState::GameOver => game_over().map_val(|| MainMenu).continue_(),
                GameState::Prompt(prompt_witness) => prompt(prompt_witness).map(Playing).continue_(),
                GameState::Running(running) => game_instance_component(running).continue_(),
            },
        })
    })
}
