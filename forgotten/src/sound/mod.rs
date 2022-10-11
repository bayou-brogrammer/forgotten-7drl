use serde::{Deserialize, Serialize};

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
    LifeStealer,
}
