use crate::prelude::*;
use gridbugs::chargrid::text::StyledString;

pub fn action_error_message(action_error: ActionError) -> StyledString {
    let style = Style::plain_text();
    let string = match action_error {
        ActionError::Message(msg) => msg,
        // ActionError::WalkIntoSolidCell => "You can't walk there!".to_string(),
        // ActionError::CannotAffordUpgrade => "You can't afford that!".to_string(),
        // ActionError::NoItemToGet => "There is no item here!".to_string(),
        // ActionError::NoWeaponInSlot(slot) => format!("No weapon in slot {}!", slot.number()),
        // ActionError::WeaponOutOfAmmo(name) => {
        //     format!("{} is out of ammo!", ui::weapon_name_text(name).string)
        // }
    };
    StyledString { string, style }
}
