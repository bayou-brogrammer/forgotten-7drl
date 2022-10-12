use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref GAME_MUSIC: Mutex<Vec<Music>> = Mutex::new(vec![Music::Gameplay0]);
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Music {
    Gameplay0,
    // Gameplay1,
    // Gameplay2,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum SoundEffect {
    // Terrain
    DoorOpen,
    DoorClose,

    // Actions
    Heal,
    Die,

    // Weapon
    Punch,
    Rifle,
    Railgun,
    FiftyCal,
    Chainsaw,
    CattleProd,
    Leecher,
    Pistol,
}
