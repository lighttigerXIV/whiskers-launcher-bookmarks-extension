use whiskers_launcher_rs::api::extensions::get_extension_dir;

use crate::EXTENSION_ID;

pub fn get_icon(name: impl Into<String>) -> String {
    let name = name.into();
    let mut path = get_extension_dir(EXTENSION_ID).unwrap();
    path.push(format!("src/resources/icons/{}", &name));

    return path.into_os_string().into_string().unwrap();
}
