use std::path::PathBuf;

use whiskers_launcher_core::features::extensions::get_extension_dir;

use crate::ID;

pub fn get_icon_path(name: impl Into<String>) -> PathBuf {
    let mut path = get_extension_dir(ID).expect("Error getting extension directory");
    path.push(format!("src/icons/{}.svg", &name.into()));
    path
}
