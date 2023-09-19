use chrono::Local;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, MultiSelect};
use dotenv::dotenv;
use porcupine::{Porcupine, PorcupineBuilder};
use pv_recorder::{PvRecorder, PvRecorderBuilder};
use std::{
    collections::HashMap,
    env,
    str::FromStr,
    sync::atomic::{AtomicBool, Ordering},
};

mod utils;
use utils::{audio_device_list, language_list, pv_keyword_paths, pv_model_paths};

mod keywords;
use keywords::Keywords;

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

    let audio_devices: Vec<String> = audio_device_list();
    let mut audio_selection: FuzzySelect = FuzzySelect::with_theme(&theme);
    let audio_device_index: i32 = audio_selection
        .with_prompt("Choose audio device input")
        .items(&audio_devices)
        .interact()
        .unwrap() as i32;

    let languages: HashMap<String, String> = language_list();
    let language_keys: Vec<String> = languages.keys().map(|s| s.to_string()).collect();
    let mut language_selection: FuzzySelect = FuzzySelect::with_theme(&theme);
    let language_index: usize = language_selection
        .with_prompt("Choose language")
        .items(&language_keys)
        .interact()
        .unwrap();
    let language: String = languages.values().nth(language_index).unwrap().to_string();

    let mut selections: MultiSelect = MultiSelect::with_theme(&theme);
    let keywords_index: Vec<usize> = selections
        .with_prompt("Choose keywords")
        .items(&Keywords::options())
        .interact()
        .unwrap();
    let keywords: Vec<Keywords> = keywords_index
        .iter()
        .map(|&index| Keywords::from_str(Keywords::options()[index]).unwrap())
        .collect();

    porcupine(audio_device_index, &language, keywords);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords_from_str() {
        assert_eq!(Keywords::from_str("position"), Ok(Keywords::Position));
        assert_eq!(Keywords::from_str("error"), Err(()));
    }
}
