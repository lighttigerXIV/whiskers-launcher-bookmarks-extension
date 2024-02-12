use std::path::PathBuf;

use actions::handle_actions;
use results::handle_results;
use whiskers_launcher_rs::api::extensions::{self, get_extension_context};

mod actions;
mod bookmarks;
mod groups;
mod resources;
mod results;

pub const EXTENSION_ID: &str = "lighttigerxiv/bookmarks";

pub fn get_config_dir() -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push("lighttigerxiv-wl-bookmarks");

    path
}

fn main() {
    let context = get_extension_context().unwrap();

    match context.action {
        extensions::Action::GetResults => handle_results(context.to_owned()),
        extensions::Action::RunAction => handle_actions(context.to_owned()),
    }
}
