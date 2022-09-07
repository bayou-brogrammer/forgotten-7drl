use crate::prelude::*;
use gridbugs::storage::{format, Storage};

#[derive(Serialize, Deserialize)]
pub struct GameInstanceStorable {
    // stars: Stars,
    pub running_game: RunningGame,
    pub current_music: Option<Music>,
}

impl GameInstanceStorable {
    pub fn into_game_instance(self) -> (GameInstance, witness::Running) {
        let Self { running_game, current_music } = self;
        let (scope, running) = running_game.into_game();
        (GameInstance { scope, current_music }, running)
    }
}

pub struct AppStorage {
    pub handle: Storage,
    pub config_key: String,
    pub controls_key: String,
    pub save_game_key: String,
}

impl AppStorage {
    const SAVE_GAME_STORAGE_FORMAT: format::Bincode = format::Bincode;
    const CONFIG_STORAGE_FORMAT: format::JsonPretty = format::JsonPretty;
    const CONTROLS_STORAGE_FORMAT: format::JsonPretty = format::JsonPretty;

    pub fn save_game(&mut self, instance: &GameInstanceStorable) {
        let result = self.handle.store(&self.save_game_key, &instance, Self::SAVE_GAME_STORAGE_FORMAT);
        if let Err(e) = result {
            use gridbugs::storage::{StoreError, StoreRawError};
            match e {
                StoreError::FormatError(e) => log::error!("Failed to format save file: {}", e),
                StoreError::Raw(e) => match e {
                    StoreRawError::IoError(e) => {
                        log::error!("Error while writing save data: {}", e)
                    }
                },
            }
        }
    }

    pub fn load_game(&self) -> Option<GameInstanceStorable> {
        let result = self
            .handle
            .load::<_, GameInstanceStorable, _>(&self.save_game_key, Self::SAVE_GAME_STORAGE_FORMAT);
        match result {
            Err(e) => {
                use gridbugs::storage::{LoadError, LoadRawError};
                match e {
                    LoadError::FormatError(e) => log::error!("Failed to parse save file: {}", e),
                    LoadError::Raw(e) => match e {
                        LoadRawError::IoError(e) => {
                            log::error!("Error while reading save data: {}", e)
                        }
                        LoadRawError::NoSuchKey => (),
                    },
                }
                None
            }
            Ok(instance) => Some(instance),
        }
    }

    pub fn clear_game(&mut self) {
        if self.handle.exists(&self.save_game_key) {
            if let Err(e) = self.handle.remove(&self.save_game_key) {
                use gridbugs::storage::RemoveError;
                match e {
                    RemoveError::IoError(e) => {
                        log::error!("Error while removing data: {}", e)
                    }
                    RemoveError::NoSuchKey => (),
                }
            }
        }
    }

    pub fn save_config(&mut self, config: &AppConfig) {
        let result = self.handle.store(&self.config_key, &config, Self::CONFIG_STORAGE_FORMAT);
        if let Err(e) = result {
            use gridbugs::storage::{StoreError, StoreRawError};
            match e {
                StoreError::FormatError(e) => log::error!("Failed to format config: {}", e),
                StoreError::Raw(e) => match e {
                    StoreRawError::IoError(e) => {
                        log::error!("Error while writing config: {}", e)
                    }
                },
            }
        }
    }

    pub fn load_config(&self) -> Option<AppConfig> {
        let result = self.handle.load::<_, AppConfig, _>(&self.config_key, Self::CONFIG_STORAGE_FORMAT);
        match result {
            Err(e) => {
                use gridbugs::storage::{LoadError, LoadRawError};
                match e {
                    LoadError::FormatError(e) => log::error!("Failed to parse config file: {}", e),
                    LoadError::Raw(e) => match e {
                        LoadRawError::IoError(e) => {
                            log::error!("Error while reading config: {}", e)
                        }
                        LoadRawError::NoSuchKey => (),
                    },
                }
                None
            }
            Ok(instance) => Some(instance),
        }
    }

    pub fn save_controls(&mut self, controls: &Controls) {
        let result = self.handle.store(&self.controls_key, &controls, Self::CONTROLS_STORAGE_FORMAT);
        if let Err(e) = result {
            use gridbugs::storage::{StoreError, StoreRawError};
            match e {
                StoreError::FormatError(e) => log::error!("Failed to format controls: {}", e),
                StoreError::Raw(e) => match e {
                    StoreRawError::IoError(e) => {
                        log::error!("Error while writing controls: {}", e)
                    }
                },
            }
        }
    }

    pub fn load_controls(&self) -> Option<Controls> {
        let result = self.handle.load::<_, Controls, _>(&self.controls_key, Self::CONTROLS_STORAGE_FORMAT);
        match result {
            Err(e) => {
                use gridbugs::storage::{LoadError, LoadRawError};
                match e {
                    LoadError::FormatError(e) => {
                        log::error!("Failed to parse controls file: {}", e)
                    }
                    LoadError::Raw(e) => match e {
                        LoadRawError::IoError(e) => {
                            log::error!("Error while reading controls: {}", e)
                        }
                        LoadRawError::NoSuchKey => (),
                    },
                }
                None
            }
            Ok(instance) => Some(instance),
        }
    }
}
