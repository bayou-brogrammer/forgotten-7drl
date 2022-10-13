use crate::prelude::*;

// Toughness:
//   1. Extra weapon slot
//   2. Double HP
//
// Accuracy:
//   1. Reduce hull pen chance to half
//   2. Deal double damage
//
// Endurance:
//   1. Half effect of vacumm pull
//   2. Double oxygen
//

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpgradeTable {
    pub toughness: Option<UpgradeLevel>,
    pub accuracy: Option<UpgradeLevel>,
    pub endurance: Option<UpgradeLevel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpgradeLevel {
    Level1,
    Level2,
    Level3,
}

impl UpgradeLevel {
    pub const fn cost(self) -> u32 {
        match self {
            Self::Level1 => 5,
            Self::Level2 => 10,
            Self::Level3 => 20,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpgradeType {
    Toughness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Upgrade {
    pub typ: UpgradeType,
    pub level: UpgradeLevel,
}
