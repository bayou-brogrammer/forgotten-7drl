use crate::prelude::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    static ref LOG: Mutex<Vec<Message>> = Mutex::new(vec![]);
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Message {
    Heal,
    Intro,
    Descend,
    PlayerDies,
    PlayerStunned,
    DoomBotExplodes,
    TakeCredit(u32),
    EnemyDies(NpcType),
    EnemyStunned(NpcType),
    AlertedNearbyEntities,
    EnemyHitPlayer(NpcType),
    EquipWeapon(WeaponType),
    EnemySlammedIntoWall(NpcType),
    PlayerHitEnemy { enemy: NpcType, weapon: WeaponType },
}

pub fn append_entry(msg: Message) {
    LOG.lock().push(msg);
}

pub fn get_log() -> Vec<Message> {
    LOG.lock().iter().cloned().collect::<Vec<_>>()
}

pub fn clear_log() {
    LOG.lock().clear();
}
