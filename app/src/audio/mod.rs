use crate::prelude::*;
use gridbugs::audio::{Audio as Sound, AudioHandle, AudioPlayer};
use maplit::hashmap;
use std::collections::HashMap;

pub type AppHandle = Option<AudioHandle>;
pub type AppAudioPlayer = Option<AudioPlayer>;

pub fn game_music_to_audio(music: Music) -> Audio {
    match music {
        Music::Gameplay0 => Audio::Gameplay0,
        //     Music::Gameplay1 => Audio::Gameplay1,
        //     Music::Gameplay2 => Audio::Gameplay2,
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Audio {
    Menu,
    Gameplay0,
    Explosion,
    SoundEffect(SoundEffect),
}

pub struct AudioTable {
    map: Option<HashMap<Audio, Sound>>,
}

impl AudioTable {
    pub fn new(audio_player: &AppAudioPlayer) -> Self {
        use audio_data::*;
        let map = audio_player.as_ref().map(|audio_player| {
            hashmap![
                Audio::Menu => audio_player.load_sound(&MENU),
                Audio::Gameplay0 => audio_player.load_sound(&GAMEPLAY0),

                Audio::Explosion => audio_player.load_sound(&EXPLOSION),
                Audio::SoundEffect(SoundEffect::Die) => audio_player.load_sound(&DIE),
                Audio::SoundEffect(SoundEffect::Heal) => audio_player.load_sound(&HEAL),
                Audio::SoundEffect(SoundEffect::DoorOpen) => audio_player.load_sound(&DOOR_OPEN),
                Audio::SoundEffect(SoundEffect::DoorClose) => audio_player.load_sound(&DOOR_CLOSE),

                // Wpns
                Audio::SoundEffect(SoundEffect::Railgun) => audio_player.load_sound(&RAILGUN),
                Audio::SoundEffect(SoundEffect::Punch) => audio_player.load_sound(&PUNCH),
                Audio::SoundEffect(SoundEffect::CattleProd) => audio_player.load_sound(&ZAP),
                Audio::SoundEffect(SoundEffect::FiftyCal) => audio_player.load_sound(&SHOTGUN),
                Audio::SoundEffect(SoundEffect::Chainsaw) => audio_player.load_sound(&CHAINSAW),
                Audio::SoundEffect(SoundEffect::Leecher) => audio_player.load_sound(&LIFE_STEALER),
            ]
        });

        Self { map }
    }

    pub fn get(&self, audio: Audio) -> Option<&Sound> {
        self.map.as_ref().map(|map| map.get(&audio).unwrap())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
/// Audio State
//////////////////////////////////////////////////////////////////////////////////////////

pub struct AudioState {
    music_volume: f32,
    music_handle: AppHandle,
    audio_table: AudioTable,
    audio_player: AppAudioPlayer,
    music_volume_multiplier: f32,
}

impl AudioState {
    pub fn new(audio_player: AppAudioPlayer) -> Self {
        let audio_table = AudioTable::new(&audio_player);
        Self { audio_player, audio_table, music_handle: None, music_volume: 1., music_volume_multiplier: 1. }
    }

    pub fn play_once(&self, audio: Audio, volume: f32) {
        log::info!("Playing audio {:?} at volume {:?}", audio, volume);
        if let Some(sound) = self.audio_table.get(audio) {
            if let Some(audio_player) = self.audio_player.as_ref() {
                let handle = audio_player.play(sound);
                handle.set_volume(volume);
                handle.background();
            }
        }
    }

    pub fn loop_music(&mut self, audio: Audio, volume: f32) {
        log::info!("Looping audio {:?} at volume {:?}", audio, volume);
        if let Some(sound) = self.audio_table.get(audio) {
            if let Some(audio_player) = self.audio_player.as_ref() {
                let handle = audio_player.play_loop(sound);
                handle.set_volume(volume * self.music_volume_multiplier);
                self.music_handle = Some(handle);
                self.music_volume = volume;
            }
        }
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume;
        if let Some(music_handle) = self.music_handle.as_mut() {
            music_handle.set_volume(volume * self.music_volume_multiplier);
        }
    }

    pub fn set_music_volume_multiplier(&mut self, music_volume_multiplier: f32) {
        self.music_volume_multiplier = music_volume_multiplier;
        if let Some(music_handle) = self.music_handle.as_mut() {
            music_handle.set_volume(self.music_volume * self.music_volume_multiplier);
        }
    }
}
