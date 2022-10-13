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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Npc {
    pub drop_chance: u8,
    pub npc_type: NpcType,
    pub weapon: Option<Weapon>,
    pub move_to: Option<Coord>,
    pub disposition: Disposition,
}
