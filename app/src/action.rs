use crate::prelude::*;
use gridbugs::chargrid::text::StyledString;

pub fn action_error_message(action_error: ActionError) -> StyledString {
    let style = Style::plain_text();
    let ActionError::Message(string) = action_error;
    StyledString { string, style }
}
