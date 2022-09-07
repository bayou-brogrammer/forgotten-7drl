pub enum ActionError {
    Message(String),
}

impl ActionError {
    pub fn err_msg<T>(s: &str) -> Result<T, Self> {
        Err(Self::Message(s.to_string()))
    }

    pub fn err_cant_walk_there<T>() -> Result<T, Self> {
        Self::err_msg("You can't walk there!")
    }
}
