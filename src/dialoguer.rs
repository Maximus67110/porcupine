use crate::keywords::Keywords;
use crate::utils::{audio_device_list, language_list};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, MultiSelect};
use std::collections::HashMap;
use std::str::FromStr;

pub fn audio_select() -> i32 {
    let theme: ColorfulTheme = ColorfulTheme::default();
    let audio_devices: Vec<String> = audio_device_list();

    FuzzySelect::with_theme(&theme)
        .with_prompt("Choose audio device input")
        .items(&audio_devices)
        .interact()
        .expect("Failed to select an audio device") as i32
}

pub fn language_select() -> String {
    let theme: ColorfulTheme = ColorfulTheme::default();
    let languages: HashMap<String, String> = language_list();
    let language_keys: Vec<String> = languages.keys().cloned().collect();

    let language_index: usize = FuzzySelect::with_theme(&theme)
        .with_prompt("Choose language")
        .items(&language_keys)
        .interact()
        .expect("Failed to select a language");

    languages.values().nth(language_index).unwrap().to_string()
}

pub fn keywords_select() -> Vec<Keywords> {
    let theme: ColorfulTheme = ColorfulTheme::default();
    let keyword_options: Vec<&str> = Keywords::options();
    let keywords_index: Vec<usize> = MultiSelect::with_theme(&theme)
        .with_prompt("Choose keywords")
        .items(&Keywords::options())
        .interact()
        .unwrap();

    let selected_keywords: Vec<Keywords> = keywords_index
        .iter()
        .map(|&index| Keywords::from_str(keyword_options[index]).expect("Invalid keyword"))
        .collect();

    selected_keywords
}
