use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;

static OVERLAY_MUSIC: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/overlay_music");
static SOUND_EFFECTS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/sound_effects");

lazy_static! {
    pub static ref GAMEPLAY0: &'static [u8] = OVERLAY_MUSIC.get_file("level_1.ogg").unwrap().contents();
    pub static ref GAMEPLAY1: &'static [u8] = OVERLAY_MUSIC.get_file("level_2.ogg").unwrap().contents();
    pub static ref GAMEPLAY2: &'static [u8] = OVERLAY_MUSIC.get_file("level_3.ogg").unwrap().contents();
    pub static ref GAMEPLAY3: &'static [u8] = OVERLAY_MUSIC.get_file("level_4.ogg").unwrap().contents();
    pub static ref GAMEPLAY4: &'static [u8] = OVERLAY_MUSIC.get_file("level_5.ogg").unwrap().contents();
    pub static ref MENU: &'static [u8] = OVERLAY_MUSIC.get_file("menu.ogg").unwrap().contents();
    pub static ref END_TEXT_SAD: &'static [u8] = OVERLAY_MUSIC.get_file("sad_ending.ogg").unwrap().contents();
    pub static ref END_TEXT_HAPPY: &'static [u8] =
        OVERLAY_MUSIC.get_file("happy_ending.ogg").unwrap().contents();
    // Sound Effects
    pub static ref DIE: &'static [u8] = SOUND_EFFECTS.get_file("die.ogg").unwrap().contents();
    pub static ref DOOR_OPEN: &'static [u8] = SOUND_EFFECTS.get_file("door_open.ogg").unwrap().contents();
    pub static ref DOOR_CLOSE: &'static [u8] = SOUND_EFFECTS.get_file("door_close.ogg").unwrap().contents();
    // Weapon Sounds
    pub static ref ZAP: &'static [u8] = SOUND_EFFECTS.get_file("zap.ogg").unwrap().contents();
    pub static ref HEAL: &'static [u8] = SOUND_EFFECTS.get_file("heal.ogg").unwrap().contents();
    pub static ref PUNCH: &'static [u8] = SOUND_EFFECTS.get_file("punch.ogg").unwrap().contents();
    pub static ref SHOTGUN: &'static [u8] = SOUND_EFFECTS.get_file("shotgun.ogg").unwrap().contents();
    pub static ref RAILGUN: &'static [u8] = SOUND_EFFECTS.get_file("railgun.ogg").unwrap().contents();
    pub static ref CHAINSAW: &'static [u8] = SOUND_EFFECTS.get_file("chainsaw.ogg").unwrap().contents();
    pub static ref EXPLOSION: &'static [u8] = SOUND_EFFECTS.get_file("explosion.ogg").unwrap().contents();
    pub static ref LIFE_STEALER: &'static [u8] = SOUND_EFFECTS.get_file("heal_gun.ogg").unwrap().contents();
}

// Overlay Music
// pub const GAMEPLAY0: &[u8] = OVERLAY_MUSIC.get_file("level_1.ogg").unwrap().contents();
// pub const GAMEPLAY1: &[u8] = OVERLAY_MUSIC.get_file("level_2.ogg").unwrap().contents();
// pub const GAMEPLAY2: &[u8] = OVERLAY_MUSIC.get_file("level_3.ogg").unwrap().contents();
// pub const GAMEPLAY3: &[u8] = OVERLAY_MUSIC.get_file("level_4.ogg").unwrap().contents();
// pub const GAMEPLAY4: &[u8] = OVERLAY_MUSIC.get_file("level_5.ogg").unwrap().contents();

// pub const MENU: &[u8] = OVERLAY_MUSIC.get_file("menu.ogg").unwrap().contents();
// pub const END_TEXT_SAD: &[u8] = OVERLAY_MUSIC.get_file("sad_ending.ogg").unwrap().contents();
// pub const END_TEXT_HAPPY: &[u8] = OVERLAY_MUSIC.get_file("happy_ending.ogg").unwrap().contents();

// Sound Effects
// pub const DIE: &[u8] = SOUND_EFFECTS.get_file("die.ogg").unwrap().contents();
// pub const DOOR_OPEN: &[u8] = SOUND_EFFECTS.get_file("door_open.ogg").unwrap().contents();
// pub const DOOR_CLOSE: &[u8] = SOUND_EFFECTS.get_file("door_close.ogg").unwrap().contents();

// Weapon Sounds
// pub const ZAP: &[u8] = SOUND_EFFECTS.get_file("zap.ogg").unwrap().contents();
// pub const HEAL: &[u8] = SOUND_EFFECTS.get_file("heal.ogg").unwrap().contents();
// pub const PUNCH: &[u8] = SOUND_EFFECTS.get_file("punch.ogg").unwrap().contents();
// pub const SHOTGUN: &[u8] = SOUND_EFFECTS.get_file("shotgun.ogg").unwrap().contents();
// pub const RAILGUN: &[u8] = SOUND_EFFECTS.get_file("railgun.ogg").unwrap().contents();
// pub const CHAINSAW: &[u8] = SOUND_EFFECTS.get_file("chainsaw.ogg").unwrap().contents();
// pub const EXPLOSION: &[u8] = SOUND_EFFECTS.get_file("explosion.ogg").unwrap().contents();
// pub const LIFE_STEALER: &[u8] = SOUND_EFFECTS.get_file("heal_gun.ogg").unwrap().contents();
