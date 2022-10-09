use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcType {
    MiniBot,
    SecBot,
    RoboCop,
    DoomBot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Disposition {
    Hostile,
    Afraid,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Npc {
    pub disposition: Disposition,
}
