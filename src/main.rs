//REQUIRED FOR EXTENSIONS ON WINDOWS
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod actions;
pub mod icons;
pub mod paths;
pub mod results;
pub mod settings;

use actions::handle_actions;
use results::handle_results;
use whiskers_launcher_rs::api::extensions::{get_extension_request, ActionContext};

pub const EXTENSION_ID: &str = "lighttigerxiv/bookmarks";

#[tokio::main]
async fn main() {
    let request = get_extension_request();

    match request.action_context {
        ActionContext::ResultsRequest => handle_results(request.to_owned()),
        ActionContext::RunAction => handle_actions(request.to_owned()).await,
    }
}
