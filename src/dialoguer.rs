use crate::keywords::Keywords;
use crate::utils::{audio_device_list, language_list};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, MultiSelect};
use std::collections::HashMap;
use std::str::FromStr;

pub fn select_audio_device() -> i32 {
    let audio_devices: Vec<String> = audio_device_list();

    let theme: ColorfulTheme = ColorfulTheme::default();
    let audio_device_index: i32 = FuzzySelect::with_theme(&theme)
        .with_prompt("Choose audio device input")
        .items(&audio_devices)
        .interact()
        .unwrap() as i32;

    audio_device_index
}

pub fn select_language() -> String {
    let languages: HashMap<String, String> = language_list();
    let language_keys: Vec<String> = languages.keys().map(|s| s.to_string()).collect();

    let theme: ColorfulTheme = ColorfulTheme::default();
    let language_index: usize = FuzzySelect::with_theme(&theme)
        .with_prompt("Choose language")
        .items(&language_keys)
        .interact()
        .unwrap();
    let language: String = languages.values().nth(language_index).unwrap().to_string();

    language
}

pub fn select_keywords() -> Vec<Keywords> {
    let theme: ColorfulTheme = ColorfulTheme::default();
    let keywords_index: Vec<usize> = MultiSelect::with_theme(&theme)
        .with_prompt("Choose keywords")
        .items(&Keywords::options())
        .interact()
        .unwrap();
    let keywords: Vec<Keywords> = keywords_index
        .iter()
        .map(|&index| Keywords::from_str(Keywords::options()[index]).unwrap())
        .collect();

    keywords
}
