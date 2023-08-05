use std::{
    collections::HashMap,
    fs::{read_dir, ReadDir},
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

pub fn pv_keyword_paths() -> HashMap<String, String> {
    let pv_platform: String = pv_platform();
    let keyword_file_pattern: String = format!("_{pv_platform}.ppn");

    let dir: PathBuf = PathBuf::from("./src/keyword");

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
