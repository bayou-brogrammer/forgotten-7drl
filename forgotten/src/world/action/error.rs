use crate::{RangedWeaponSlot, WeaponType};

pub enum ActionError {
    Message(String),
    Weapon(String, WeaponType),
}

impl ActionError {
    pub fn err_msg<T>(s: &str) -> Result<T, Self> {
        Err(Self::Message(s.to_string()))
    }

    pub fn wpn_err<T>(s: &str, wpn_name: WeaponType) -> Result<T, Self> {
        Err(Self::Weapon(s.to_string(), wpn_name))
    }

    pub fn err_cant_walk_there<T>() -> Result<T, Self> {
        Self::err_msg("You can't walk there!")
    }

    pub fn no_item_there<T>() -> Result<T, Self> {
        Self::err_msg("There is no item here!")
    }

    pub fn no_weapon_in_slot<T>(slot: RangedWeaponSlot) -> Result<T, Self> {
        Self::err_msg(&format!("There is no weapon in slot {}!", slot.index() + 1))
    }

    pub fn out_of_ammo<T>(name: WeaponType) -> Result<T, Self> {
        Self::err_msg(&format!("{} is out of ammo!", name.to_string()))
    }
}
