use std::{env, str::FromStr, sync::atomic::{AtomicBool, Ordering}, collections::HashMap};
use pv_recorder::{PvRecorder, PvRecorderBuilder, PvRecorderError};
use dialoguer::{FuzzySelect, MultiSelect, theme::ColorfulTheme};
use porcupine::{Porcupine, PorcupineBuilder};
use dotenv::dotenv;
use chrono::Local;

mod utils;
use utils::pv_keyword_paths;

static LISTENING: AtomicBool = AtomicBool::new(false);

fn porcupine(
    audio_device_index: i32,
    keywords: Vec<Keywords>,
) {
    let default_keyword_paths :HashMap<String, String> = pv_keyword_paths();
    let keyword_paths: Vec<String> = keywords
        .iter()
        .map(|keyword| {
            default_keyword_paths
                .get(keyword.to_str())
                .expect("Unable to find keyword file for specified keyword")
        })
        .cloned()
        .collect::<Vec<_>>();
    let mut porcupine_builder: PorcupineBuilder = PorcupineBuilder::new_with_keyword_paths(env::var("ACCESS_TOKEN").unwrap(), &keyword_paths);
    porcupine_builder.model_path("porcupine_params_fr.pv");

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

    let mut selection: FuzzySelect = FuzzySelect::with_theme(&theme);
    let audio_devices: Result<Vec<String>, PvRecorderError> = PvRecorderBuilder::default()
        .get_available_devices();
    match audio_devices {
        Ok(audio_devices) => {
            for (_usize, name) in audio_devices.iter().enumerate() {
                selection.item(name);
            }
        }
        Err(err) => panic!("Failed to get audio devices: {}", err),
    };
    let audio_device_index: i32 = selection.with_prompt("Choose audio device input").interact().unwrap() as i32;

    let mut selections: MultiSelect = MultiSelect::with_theme(&theme);
    selections.items(&Keywords::options());
    let keywords_index: Vec<usize> = selections.with_prompt("Choose keywords").interact().unwrap();
    let keywords: Vec<Keywords> = keywords_index
        .iter()
        .map(|&index| Keywords::from_str(Keywords::options()[index]).unwrap())
        .collect();

    porcupine(
        audio_device_index,
        keywords,
    );
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
        vec![
            "position",
        ]
    }
}
