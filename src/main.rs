use chrono::Local;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, MultiSelect};
use dotenv::dotenv;
use porcupine::{Porcupine, PorcupineBuilder};
use pv_recorder::{PvRecorder, PvRecorderBuilder, PvRecorderError};
use std::{
    collections::HashMap,
    env,
    fs::{read_dir, DirEntry, Metadata, ReadDir},
    path::PathBuf,
    str::FromStr,
    sync::atomic::{AtomicBool, Ordering},
};

mod utils;
use utils::{pv_keyword_paths, pv_model_paths};

static LISTENING: AtomicBool = AtomicBool::new(false);

fn porcupine(audio_device_index: i32, language: &String, keywords: Vec<Keywords>) {
    let default_keyword_paths: HashMap<String, String> = pv_keyword_paths(language);
    let keyword_paths: Vec<String> = keywords
        .iter()
        .map(|keyword| {
            default_keyword_paths
                .get(keyword.to_str())
                .expect("Unable to find keyword file for specified keyword")
        })
        .cloned()
        .collect::<Vec<_>>();
    let mut porcupine_builder: PorcupineBuilder =
        PorcupineBuilder::new_with_keyword_paths(env::var("ACCESS_TOKEN").unwrap(), &keyword_paths);
    let default_model_paths: HashMap<String, String> = pv_model_paths();
    if let Some(model_path) = default_model_paths.get(language) {
        porcupine_builder.model_path(model_path);
    }

    let porcupine: Porcupine = porcupine_builder
        .init()
        .expect("Failed to create Porcupine");

    let recorder: PvRecorder = PvRecorderBuilder::new(porcupine.frame_length() as i32)
        .device_index(audio_device_index)
        .init()
        .expect("Failed to initialize pvrecorder");
    recorder.start().expect("Failed to start audio recording");

    LISTENING.store(true, Ordering::SeqCst);
    ctrlc::set_handler(|| {
        LISTENING.store(false, Ordering::SeqCst);
    })
    .expect("Unable to setup signal handler");

    println!("Listening for wake words...");

    while LISTENING.load(Ordering::SeqCst) {
        let frame: Vec<i16> = recorder.read().expect("Failed to read audio frame");

        let keyword_index: i32 = porcupine.process(&frame).unwrap();
        if keyword_index >= 0 {
            println!(
                "[{}] Detected {:?}",
                Local::now().format("%F %T"),
                keywords[keyword_index as usize]
            );
        }
    }

    println!("\nStopping...");
    recorder.stop().expect("Failed to stop audio recording");
}

fn main() {
    dotenv().ok();
    env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN not found");
    let theme: ColorfulTheme = ColorfulTheme::default();

    let mut audio_selection: FuzzySelect = FuzzySelect::with_theme(&theme);
    let audio_devices: Result<Vec<String>, PvRecorderError> =
        PvRecorderBuilder::default().get_available_devices();
    match audio_devices {
        Ok(audio_devices) => {
            for (_usize, name) in audio_devices.iter().enumerate() {
                audio_selection.item(name);
            }
        }
        Err(err) => panic!("Failed to get audio devices: {}", err),
    };
    let audio_device_index: i32 = audio_selection
        .with_prompt("Choose audio device input")
        .interact()
        .unwrap() as i32;

    let mut languages: Vec<String> = Vec::new();
    let dir: PathBuf = PathBuf::from("./src/keyword");
    let dir_entries: ReadDir = read_dir(&dir)
        .unwrap_or_else(|_| panic!("Can't find default keyword_files dir: {}", dir.display()));
    for entry in dir_entries {
        let entry: DirEntry = entry.unwrap();
        let metadata: Metadata = entry.metadata().unwrap();
        if metadata.is_dir() {
            if let Some(entry_name) = entry.file_name().to_str() {
                languages.push(entry_name.to_string());
            }
        }
    }
    let mut language_selection: FuzzySelect = FuzzySelect::with_theme(&theme);
    let language_index: usize = language_selection
        .with_prompt("Choose language")
        .items(&languages)
        .interact()
        .unwrap();
    let language: &String = &languages[language_index];

    let mut selections: MultiSelect = MultiSelect::with_theme(&theme);
    selections.items(&Keywords::options());
    let keywords_index: Vec<usize> = selections
        .with_prompt("Choose keywords")
        .interact()
        .unwrap();
    let keywords: Vec<Keywords> = keywords_index
        .iter()
        .map(|&index| Keywords::from_str(Keywords::options()[index]).unwrap())
        .collect();

    porcupine(audio_device_index, language, keywords);
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Keywords {
    Position,
}

impl FromStr for Keywords {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "position" => Ok(Self::Position),
            _ => Err(()),
        }
    }
}

impl Keywords {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Position => "position",
        }
    }

    pub fn options() -> Vec<&'static str> {
        vec!["position"]
    }
}
