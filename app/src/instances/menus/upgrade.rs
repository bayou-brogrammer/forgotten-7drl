use gridbugs::chargrid::{
    menu::{self, Menu},
    text::StyledString,
};

use super::menu_style;
use crate::{instances::popup, prelude::*};

fn upgrade_identifier(upgrade: Upgrade) -> String {
    let name = match upgrade.typ {
        UpgradeType::Toughness => "Toughness",
    };
    let level = match upgrade.level {
        UpgradeLevel::Level1 => "1",
        UpgradeLevel::Level2 => "2",
        UpgradeLevel::Level3 => "3",
    };
    let price = upgrade.level.cost();
    format!("{} {} (${})", name, level, price)
}

fn upgrade_description(upgrade: &Upgrade) -> &'static str {
    use {UpgradeLevel::*, UpgradeType::*};
    match upgrade {
        Upgrade { typ: Toughness, level: Level1 } => {
            "Toughness 1: Strong Back\nGain a third ranged weapon slot."
        }
        Upgrade { typ: Toughness, level: Level2 } => "Toughness 2: Hardy\nDouble your maximum health.",
        Upgrade { typ: Toughness, level: Level3 } => {
            "Toughness 3: Immune to Explosions + Explosive Rounds ;)"
        }
    }
}

struct UpgradeMenuDecorated {
    menu: Menu<Upgrade>,
}
impl UpgradeMenuDecorated {
    const MENU_Y_OFFSET: i32 = 4;
    const TEXT_STYLE: Style = Style::new().with_bold(false).with_foreground(Rgba32::new_grey(255));
    const SIZE: Size = Size::new_u16(33, 13);

    fn text(ctx: Ctx, fb: &mut FrameBuffer, string: String) {
        StyledString { string, style: Self::TEXT_STYLE }.render(&(), ctx, fb);
    }
}

impl Component for UpgradeMenuDecorated {
    type Output = Option<Upgrade>;
    type State = GameLoopData;

    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        let instance = state.instance.as_ref().unwrap();
        let balance = instance.scope.player().credit;

        Self::text(ctx, fb, "Buy an Upgrade (escape cancels)".to_string());
        Self::text(ctx.add_y(2), fb, format!("Your balance: ${}", balance));
        self.menu.render(&(), ctx.add_y(Self::MENU_Y_OFFSET), fb);
        let description = upgrade_description(self.menu.selected());

        StyledString { string: description.to_string(), style: Self::TEXT_STYLE }
            .wrap_word()
            .cf()
            .bound_width(Self::SIZE.width())
            .render(&(), ctx.add_y(9), fb);
    }

    fn update(&mut self, _state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        self.menu.update(&mut (), ctx.add_y(Self::MENU_Y_OFFSET), event)
    }

    fn size(&self, _state: &Self::State, _ctx: Ctx) -> Size {
        Self::SIZE
    }
}

fn upgrade_menu() -> AppCF<Upgrade> {
    on_state_then(|state: &mut State| {
        let instance = state.instance.as_ref().unwrap();
        let upgrades = instance.scope.available_upgrades();

        use menu::builder::*;
        let mut builder = menu_builder().vi_keys();
        for upgrade in upgrades {
            let name = upgrade_identifier(upgrade);
            let identifier = MENU_FADE_SPEC.identifier(move |b| write!(b, "{}", name).unwrap());
            builder = builder.add_item(item(upgrade, identifier));
        }
        let menu = builder.build();
        UpgradeMenuDecorated { menu }
    })
}

fn upgrade_component(upgrade_witness: UpgradeState) -> AppCF<GameState> {
    menu_style(upgrade_menu()).menu_harness().and_then(|result| {
        on_state_then(move |state: &mut State| match result {
            Err(Close) => val_once(upgrade_witness.cancel()),
            Ok(upgrade) => {
                let instance = state.instance.as_mut().unwrap();
                if upgrade.level.cost() > instance.scope.player().credit {
                    popup("You can't afford that!".to_string()).map_val(|| upgrade_witness.cancel())
                } else {
                    val_once(upgrade_witness.commit(&mut instance.scope, upgrade))
                }
            }
        })
    })
}

pub fn try_upgrade_component(upgrade_witness: UpgradeState) -> AppCF<GameState> {
    on_state_then(move |state: &mut State| {
        let instance = state.instance.as_ref().unwrap();
        let upgrades = instance.scope.available_upgrades();
        if upgrades.is_empty() {
            popup("No remaining upgrades!".to_string()).map_val(|| upgrade_witness.cancel())
        } else {
            upgrade_component(upgrade_witness)
        }
    })
}
