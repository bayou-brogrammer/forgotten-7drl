use std::path::PathBuf;

use clap::{Parser, Subcommand};
use forgotten_app::{AppArgs, AppStorage, InitialRngSeed};
use gridbugs::{
    audio::{AudioPlayer, NativeAudioError, NativeAudioPlayer},
    storage::{FileStorage, IfDirectoryMissing, Storage},
};

mod ansi;
mod wgpu;

pub use ansi::*;
pub use wgpu::*;

const DEFAULT_SAVE_FILE: &str = "save";
const DEFAULT_CONFIG_FILE: &str = "config.json";
const DEFAULT_NEXT_TO_EXE_STORAGE_DIR: &str = "save";
const DEFAULT_CONTROLS_FILE: &str = "controls.json";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// What frontend to run the program in
    #[clap(subcommand)]
    frontend: Option<FrontEnd>,

    #[clap(short, long, value_parser, value_name = "INT")]
    rng_seed: Option<u64>,

    #[clap(long, value_parser, value_name = "PATH", default_value_t = String::from(DEFAULT_NEXT_TO_EXE_STORAGE_DIR))]
    storage_dir: String,
    #[clap(long, value_parser, value_name = "PATH", default_value_t = String::from(DEFAULT_CONFIG_FILE))]
    config_file: String,
    #[clap(long, value_parser, value_name = "PATH", default_value_t = String::from(DEFAULT_SAVE_FILE))]
    save_file: String,
    #[clap(long, value_parser, value_name = "PATH", default_value_t = String::from(DEFAULT_CONTROLS_FILE))]
    controls_file: String,

    #[clap(long, action, default_value_t = false)]
    delete_save: bool,
    #[clap(long, action, default_value_t = false)]
    delete_config: bool,
    #[clap(long, action, default_value_t = false)]
    delete_controls: bool,
    #[clap(long, action, default_value_t = false)]
    delete_all_configs: bool,

    #[clap(short, long, action, default_value_t = false)]
    new_game: bool,

    #[clap(short, long, action, default_value_t = false)]
    omniscient: bool,

    #[clap(short, long, action, default_value_t = false)]
    mute: bool,
}

#[derive(Subcommand)]
enum FrontEnd {
    /// Run program with WGPU frontend
    Wgpu(Wgpu),
    /// Run program with ANSI terminal frontend
    Ansi(AnsiTerminal),
}

fn main() {
    let Cli {
        frontend,
        rng_seed,
        storage_dir,
        controls_file,
        save_file,
        delete_save,
        new_game,
        omniscient,
        delete_config,
        delete_controls,
        delete_all_configs,
        config_file,
        mute,
    } = Cli::parse();

    let initial_rng_seed = rng_seed.map(InitialRngSeed::U64).unwrap_or(InitialRngSeed::Random);
    let mut file_storage =
        Storage::new(match FileStorage::next_to_exe(&storage_dir, IfDirectoryMissing::Create) {
            Ok(fs) => fs,
            Err(_) => {
                log::warn!(
                    "Couldn't create save dir next to executable. Will use temporary directory instead."
                );
                FileStorage::temp(PathBuf::from("rainforest").join(storage_dir), IfDirectoryMissing::Create)
                    .expect("failed to open directory")
            }
        });

    if delete_all_configs {
        delete_file(&mut file_storage, &save_file);
        delete_file(&mut file_storage, &config_file);
        delete_file(&mut file_storage, &controls_file);
    } else {
        if delete_save {
            delete_file(&mut file_storage, &save_file);
        }
        if delete_config {
            delete_file(&mut file_storage, &config_file);
        }
        if delete_controls {
            delete_file(&mut file_storage, &controls_file);
        }
    }

    let storage = AppStorage {
        handle: file_storage,
        save_game_key: save_file,
        config_key: config_file,
        controls_key: controls_file,
    };

    let audio_player = if mute {
        None
    } else {
        match NativeAudioPlayer::try_new_default_device() {
            Ok(audio_player) => Some(AudioPlayer::new(audio_player)),
            Err(NativeAudioError::FailedToCreateOutputStream) => {
                log::warn!("no output audio device - continuing without audio");
                None
            }
        }
    };
    let args = AppArgs { storage, initial_rng_seed, audio_player, omniscient, new_game };

    match frontend.unwrap_or_else(|| FrontEnd::Wgpu(Wgpu::default())) {
        FrontEnd::Wgpu(wgpu) => wgpu.run(forgotten_app::run_app(args)),
        FrontEnd::Ansi(ansi) => ansi.run(forgotten_app::run_app(args)),
    }
}

fn delete_file<K: AsRef<str>>(file_storage: &mut Storage, file: K) {
    let result = file_storage.remove(&file);
    if result.is_err() {
        log::warn!("couldn't find file to delete");
    }
}
