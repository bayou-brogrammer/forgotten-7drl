use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Music {
    Gameplay0,
    // Gameplay1,
    // Gameplay2,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum SoundEffect {
    Shotgun,
    Rifle,
    Railgun,
    GausCannon,
    LifeStealer,
    Oxidiser,
    Chainsaw,
    Punch,
    DoorOpen,
    Heal,
    Die,
}
