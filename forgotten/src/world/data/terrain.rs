use crate::prelude::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DoorState {
    Open,
    Closed,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GrassState {
    Normal,
    Crushed,
}
