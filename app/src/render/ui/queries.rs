use gridbugs::chargrid::text::StyledString;

use crate::prelude::*;

pub fn weapon_name_text(weapon_name: WeaponType) -> StyledString {
    let t = |s: &str, c| StyledString {
        string: s.to_string(),
        style: Style::new().with_foreground(c).with_bold(true),
    };
    let name = weapon_name.to_string();
    let color = match weapon_name {
        WeaponType::BareHands => Rgba32::new_grey(255),
        WeaponType::CattleProd => todo!(),
        WeaponType::Chainsaw => todo!(),
        WeaponType::Railgun => todo!(),
        WeaponType::LifeStealer => todo!(),
    };

    t(name.as_str(), color)
}

pub fn enemy_text(enemy: NpcType) -> StyledString {
    let t = |s: &str, c| StyledString {
        string: s.to_string(),
        style: Style::new().with_foreground(c).with_bold(true),
    };
    match enemy {
        NpcType::MiniBot => t("MiniBot", color::MINIBOT),
        NpcType::SecBot => t("Secbot", color::SECBOT),
        NpcType::RoboCop => t("RoboCop", color::ROBOCOP.saturating_scalar_mul_div(3, 2)),
        NpcType::DoomBot => t("DoomBot", color::DOOMBOT.saturating_scalar_mul_div(3, 2)),
    }
}
