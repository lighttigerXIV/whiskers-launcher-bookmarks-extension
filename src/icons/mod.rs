use whiskers_launcher_rs::api::extensions::get_extension_dir;

use crate::EXTENSION_ID;

pub fn get_icon_path(name: impl Into<String>) -> String {
    let mut path = get_extension_dir(EXTENSION_ID).expect("Error getting extension directory");

    path.push(format!("src/icons/{}.svg", &name.into()));

    path.into_os_string()
        .into_string()
        .expect("Error converting path to string")
}
