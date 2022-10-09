use crate::prelude::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    static ref EXTERNAL_EVENTS: Mutex<Vec<ExternalEvent>> = Mutex::new(Vec::new());
}

/// Events which the game can report back to the io layer so it can
/// respond with a sound/visual effect.
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ExternalEvent {
    Explosion(Coord),
    LoopMusic(Music),
    SoundEffect(SoundEffect),
}

pub fn add_event(ev: ExternalEvent) {
    EXTERNAL_EVENTS.lock().push(ev);
}

pub fn get_events() -> Vec<ExternalEvent> {
    EXTERNAL_EVENTS.lock().drain(..).collect()
}
