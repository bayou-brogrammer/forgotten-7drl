use super::*;

struct GameInstanceComponent(Option<Running>);

impl GameInstanceComponent {
    fn new(running: Running) -> Self {
        Self(Some(running))
    }
}

impl Component for GameInstanceComponent {
    type State = GameLoopData;
    type Output = GameLoopState;

    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        state.render(color::CURSOR, ctx, fb);
    }

    fn update(&mut self, state: &mut Self::State, _ctx: Ctx, event: Event) -> Self::Output {
        _ctx.add_size(Size::new(20, 20));
        let running = self.0.take().unwrap();
        if event.is_escape_or_start() {
            GameLoopState::Paused(running)
        } else {
            self.update(state, event, running)
        }
    }

    fn size(&self, _state: &Self::State, ctx: Ctx) -> Size {
        println!("GameInstanceComponent::size");
        ctx.bounding_box.size()
    }
}

impl GameInstanceComponent {
    fn update(&mut self, state: &mut GameLoopData, event: Event, running: state::Running) -> GameLoopState {
        let instance = state.instance.as_mut().unwrap();

        let witness = match event {
            Event::Input(input) => {
                if let Some(app_input) = state.controls.get(input) {
                    state.cursor = None;
                    let (witness, action_result) = match app_input {
                        AppInput::Get => todo!(),
                        AppInput::Wait => todo!(),
                        AppInput::Examine => todo!(),
                        AppInput::Direction(direction) => running.player_walk(&mut instance.scope, direction),
                    };

                    if let Err(action_error) = action_result {
                        state.context_message = Some(action_error_message(action_error));
                    } else {
                        state.context_message = None;
                    }
                    witness
                } else {
                    running.into_witness()
                }
            }
            Event::Tick(since_previous) => running.tick(&mut instance.scope, since_previous),
            _ => GameState::Running(running),
        };

        GameLoopState::Playing(witness)
    }
}

pub fn game_instance_component(running: state::Running) -> AppCF<GameLoopState> {
    cf(GameInstanceComponent::new(running)).some().no_peek()
}
