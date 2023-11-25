use chrono::Local;
use dotenv::dotenv;
use porcupine::{Porcupine, PorcupineBuilder};
use pv_recorder::{PvRecorder, PvRecorderBuilder};
use std::{
    collections::HashMap,
    env,
    sync::atomic::{AtomicBool, Ordering},
};

mod dialoguer;
use crate::dialoguer::{select_audio_device, select_keywords, select_language};

mod utils;
use utils::{pv_keyword_paths, pv_model_paths};

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
    let audio_device_index: i32 = select_audio_device();
    let language: String = select_language();
    let keywords: Vec<Keywords> = select_keywords();

    porcupine(audio_device_index, &language, keywords);
}
