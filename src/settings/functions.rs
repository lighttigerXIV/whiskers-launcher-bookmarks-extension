use std::fs;

use crate::paths::get_settings_path;

use super::Settings;

pub fn get_settings() -> Settings {
    let path = get_settings_path();

    if !path.parent().unwrap().exists() {
        fs::create_dir_all(&path.parent().unwrap()).expect("Error creating settings directory");
    }

    if !path.exists() {
        let default_settings = Settings {
            groups: Vec::new(),
            bookmarks: Vec::new(),
        };

        fs::write(
            &path,
            &bincode::serialize(&default_settings).expect("Error serializing settings"),
        )
        .expect("Error writing default settings");

        return default_settings.to_owned();
    }

    let bytes = fs::read(&path).expect("Error reading settings file");
    let settings = bincode::deserialize(&bytes).expect("Error deserializing settings file");
    settings
}

pub fn write_settings(settings: Settings) {
    let mut settings = settings;
    settings.groups.sort_by_key(|g| g.id.to_owned());
    settings.bookmarks.sort_by_key(|b| b.id.to_owned());

    let bytes = bincode::serialize(&settings).expect("Error serializing settings file");
    fs::write(get_settings_path(), &bytes).expect("Error writing settings file");
}
