use isolang::Language;
use pv_recorder::{PvRecorderBuilder, PvRecorderError};
use std::{
    collections::HashMap,
    fs::{read_dir, DirEntry, Metadata, ReadDir},
    path::PathBuf,
};

#[cfg(target_os = "macos")]
pub fn pv_platform() -> String {
    String::from("mac")
}

#[cfg(target_os = "windows")]
pub fn pv_platform() -> String {
    String::from("windows")
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub fn pv_platform() -> String {
    String::from("linux")
}

pub fn pv_keyword_paths(language: &String) -> HashMap<String, String> {
    let pv_platform: String = pv_platform();
    let keyword_file_pattern: String = format!("_{language}_{pv_platform}.ppn");

    let dir: PathBuf = PathBuf::from(format!("./src/keyword/{language}"));

    let mut keyword_paths: HashMap<String, String> = HashMap::new();
    let dir_entries: ReadDir = read_dir(&dir)
        .unwrap_or_else(|_| panic!("Can't find default keyword_files dir: {}", dir.display()));

    for entry in dir_entries.flatten() {
        let path: PathBuf = entry.path();
        let keyword_string: String = entry.file_name().into_string().unwrap();

        if keyword_string.contains(&keyword_file_pattern)
            && keyword_string.len() > keyword_file_pattern.len()
        {
            if let Some(keyword) = keyword_string.split('_').next() {
                keyword_paths.insert(
                    keyword.to_string(),
                    path.into_os_string().into_string().unwrap(),
                );
            }
        }
    }

    keyword_paths
}

pub fn pv_model_paths() -> HashMap<String, String> {
    let dir: PathBuf = PathBuf::from("./src/model");

    let mut model_paths: HashMap<String, String> = HashMap::new();
    let dir_entries: ReadDir = read_dir(&dir)
        .unwrap_or_else(|_| panic!("Can't find default model_files dir: {}", dir.display()));

    for entry in dir_entries.flatten() {
        let path: PathBuf = entry.path();
        let keyword_string: String = entry.file_name().into_string().unwrap();
        if let Some(language) = keyword_string.split('_').last() {
            if let Some(language) = language.split('.').next() {
                model_paths.insert(
                    language.to_string(),
                    path.into_os_string().into_string().unwrap(),
                );
            }
        }
    }

    model_paths
}

pub fn audio_device_list() -> Vec<String> {
    let mut audio_device_list: Vec<String> = Vec::new();
    let audio_devices: Result<Vec<String>, PvRecorderError> =
        PvRecorderBuilder::default().get_available_devices();
    match audio_devices {
        Ok(audio_devices) => {
            for (_usize, name) in audio_devices.iter().enumerate() {
                audio_device_list.push(name.to_string());
            }
        }
        Err(err) => panic!("Failed to get audio devices: {}", err),
    };

    audio_device_list
}

pub fn language_list() -> HashMap<String, String> {
    let mut languages: HashMap<String, String> = HashMap::new();
    let dir: PathBuf = PathBuf::from("./src/keyword");
    let dir_entries: ReadDir = read_dir(&dir)
        .unwrap_or_else(|_| panic!("Can't find default keyword_files dir: {}", dir.display()));
    for entry in dir_entries {
        let entry: DirEntry = entry.unwrap();
        let metadata: Metadata = entry.metadata().unwrap();
        if metadata.is_dir() {
            if let Some(entry_name) = entry.file_name().to_str() {
                if let Some(language_name) = Language::from_639_1(entry_name).unwrap().to_autonym()
                {
                    languages.insert(language_name.to_string(), entry_name.to_string());
                }
            }
        }
    }

    languages
}
