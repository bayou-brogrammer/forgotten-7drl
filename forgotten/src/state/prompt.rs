use super::*;

#[derive(Debug)]
pub struct Prompt {
    pub(crate) message: String,
    pub(crate) private: Private,
}

impl Prompt {
    pub fn into_witness(self) -> GameState {
        GameState::Prompt(self)
    }

    pub fn running(self) -> GameState {
        Running(self.private).into_witness()
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}
