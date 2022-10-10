use crate::prelude::*;
use gridbugs::chargrid::text::StyledString;

pub fn action_error_message(action_error: ActionError) -> StyledString {
    let style = Style::plain_text();
    let string = match action_error {
        ActionError::Message(msg) => msg,
        ActionError::Weapon(msg, wpn_name) => format!("{} {:?}", msg, weapon_name_text(wpn_name)),
    };
    StyledString { string, style }
}
