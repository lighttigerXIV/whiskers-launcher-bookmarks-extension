use std::path::PathBuf;

fn get_config_dir() -> PathBuf {
    let mut path = dirs::config_dir().expect("Error config dir");
    path.push("whiskers-launcher-bookmarks");
    path
}

pub fn get_settings_path() -> PathBuf {
    let mut path = get_config_dir();
    path.push("settings.bin");
    path
}

pub fn get_favicons_dir() -> PathBuf{
    let mut path = get_config_dir();
    path.push("favicons");
    path
}