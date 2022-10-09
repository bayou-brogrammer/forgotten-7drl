use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum OptionsMenuEntry {
    MusicVolume,
    SfxVolume,
    Back,
}

struct OptionsMenuComponent {
    menu: Menu<OptionsMenuEntry>,
}

impl Component for OptionsMenuComponent {
    type Output = Option<()>;
    type State = GameLoopData;

    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        self.menu.render(&(), ctx, fb);
        let x_offset = 14;
        let style = Style::default().with_foreground(Rgba32::new_grey(255)).with_bold(false);
        StyledString { string: format!("< {:.0}% >", state.config.music_volume * 100.), style }.render(
            &(),
            ctx.add_offset(Coord { x: x_offset, y: 0 }),
            fb,
        );
        StyledString { string: format!("< {:.0}% >", state.config.sfx_volume * 100.), style }.render(
            &(),
            ctx.add_offset(Coord { x: x_offset, y: 1 }),
            fb,
        );
    }

    fn update(&mut self, state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        let mut update_volume = |volume_delta: f32| {
            let volume = match self.menu.selected() {
                OptionsMenuEntry::MusicVolume => &mut state.config.music_volume,
                OptionsMenuEntry::SfxVolume => &mut state.config.sfx_volume,
                OptionsMenuEntry::Back => return,
            };
            *volume = (*volume + volume_delta).clamp(0., 1.);
            state.audio_state.set_music_volume(state.config.music_volume);
            state.save_config();
        };
        if let Some(input_policy) = event.input_policy() {
            match input_policy {
                InputPolicy::Left => update_volume(-0.05),
                InputPolicy::Right => update_volume(0.05),
                InputPolicy::Select => {
                    // prevent hitting enter on a menu option from closing the menu
                    if OptionsMenuEntry::Back != *self.menu.selected() {
                        return None;
                    }
                }
                _ => (),
            }
        }
        self.menu.update(&mut (), ctx, event).map(|_| ())
    }

    fn size(&self, _state: &Self::State, ctx: Ctx) -> Size {
        self.menu.size(&(), ctx) + Size::new(9, 0)
    }
}

pub fn options_menu() -> AppCF<()> {
    use OptionsMenuEntry::*;

    let mut builder = menu_builder().vi_keys();
    let add_item = |builder: &mut MenuBuilder<_>, entry, name| {
        let identifier = MENU_FADE_SPEC.identifier(move |b| write!(b, "{}", name).unwrap());
        builder.add_item_mut(item(entry, identifier));
    };

    add_item(&mut builder, MusicVolume, "Music Volume:");
    add_item(&mut builder, SfxVolume, "SFX Volume:");
    builder.add_space_mut();
    add_item(&mut builder, Back, "Back");

    let menu = builder.build();
    cf(OptionsMenuComponent { menu }).catch_escape_or_start().map(|_| ())
}
