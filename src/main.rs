use std::env;
use chrono::prelude::*;
use clap::{App, Arg, ArgGroup};
use porcupine::{BuiltinKeywords, Porcupine, PorcupineBuilder};
use pv_recorder::{PvRecorder, PvRecorderBuilder, PvRecorderError};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use dotenv::dotenv;

static LISTENING: AtomicBool = AtomicBool::new(false);

fn porcupine(
    audio_device_index: i32,
    keywords_or_paths: KeywordsOrPaths,
) {
    let mut porcupine_builder: PorcupineBuilder = match keywords_or_paths {
        KeywordsOrPaths::Keywords(ref keywords) => {
            PorcupineBuilder::new_with_keywords(env::var("ACCESS_TOKEN").unwrap(), keywords)
        }
        KeywordsOrPaths::KeywordPaths(ref keyword_paths) => {
            PorcupineBuilder::new_with_keyword_paths(env::var("ACCESS_TOKEN").unwrap(), keyword_paths)
        }
    };
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
                "[{}] Detected {}",
                Local::now().format("%F %T"),
                keywords_or_paths.get(keyword_index as usize)
            );
        }
    }

    println!("\nStopping...");
    recorder.stop().expect("Failed to stop audio recording");
}

#[derive(Clone)]
enum KeywordsOrPaths {
    Keywords(Vec<BuiltinKeywords>),
    KeywordPaths(Vec<PathBuf>),
}

impl KeywordsOrPaths {
    fn get(&self, index: usize) -> String {
        match self {
            Self::Keywords(keywords) => keywords[index].to_str().to_string(),
            Self::KeywordPaths(keyword_paths) => keyword_paths[index]
                .clone()
                .into_os_string()
                .into_string()
                .unwrap(),
        }
    }
}

fn show_audio_devices() {
    let audio_devices: Result<Vec<String>, PvRecorderError> = PvRecorderBuilder::default().get_available_devices();
    match audio_devices {
        Ok(audio_devices) => {
            for (idx, device) in audio_devices.iter().enumerate() {
                println!("index: {idx}, device name: {device:?}");
            }
        }
        Err(err) => panic!("Failed to get audio devices: {}", err),
    };
}

fn main() {
    dotenv().ok();
    env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN not found");
    let matches = App::new("Picovoice Porcupine Rust Mic Demo")
        .group(
            ArgGroup::with_name("actions_group")
                .arg("keywords")
                .arg("keyword_paths")
                .arg("show_audio_devices")
                .required(true)
                .multiple(true)
        )
        .arg(
            Arg::with_name("keywords")
                .long("keywords")
                .value_name("KEYWORDS")
                .use_delimiter(true)
                .help("Comma-separated list of default keywords for detection.")
                .takes_value(true)
                .possible_values(&BuiltinKeywords::options())
        )
        .arg(
            Arg::with_name("keyword_paths")
                .long("keyword_paths")
                .value_name("PATHS")
                .use_delimiter(true)
                .help("Comma-separated list of paths to keyword model files. If not set it will be populated from `--keywords` argument.")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("audio_device_index")
                .long("audio_device_index")
                .value_name("INDEX")
                .help("Index of input audio device.")
                .takes_value(true)
                .default_value("-1")
        )
        .arg(
            Arg::with_name("show_audio_devices")
                .long("show_audio_devices")
        )
        .get_matches();

    if matches.is_present("show_audio_devices") {
        return show_audio_devices();
    }

    let audio_device_index = matches
        .value_of("audio_device_index")
        .unwrap()
        .parse()
        .unwrap();

    let keywords_or_paths: KeywordsOrPaths = {
        if matches.is_present("keyword_paths") {
            KeywordsOrPaths::KeywordPaths(
                matches
                    .values_of("keyword_paths")
                    .unwrap()
                    .map(|path| PathBuf::from(path.to_string()))
                    .collect(),
            )
        } else if matches.is_present("keywords") {
            KeywordsOrPaths::Keywords(
                matches
                    .values_of("keywords")
                    .unwrap()
                    .flat_map(|keyword| match BuiltinKeywords::from_str(keyword) {
                        Ok(keyword) => vec![keyword],
                        Err(_) => vec![],
                    })
                    .collect(),
            )
        } else {
            panic!("Keywords or keyword paths must be specified");
        }
    };

    porcupine(
        audio_device_index,
        keywords_or_paths,
    );
}
