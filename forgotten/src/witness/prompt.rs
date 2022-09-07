use super::*;

#[derive(Debug)]
pub struct Prompt {
    pub(crate) message: String,
    pub(crate) private: Private,
}

impl Prompt {
    pub fn into_witness(self) -> Witness {
        Witness::Prompt(self)
    }

    pub fn running(self) -> Witness {
        Running(self.private).into_witness()
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}
