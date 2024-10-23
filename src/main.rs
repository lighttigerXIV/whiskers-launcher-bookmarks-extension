//REQUIRED FOR EXTENSIONS ON WINDOWS
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod icons;
pub mod paths;
pub mod results;
pub mod settings;


use commands::on_run_commands;
use results::on_get_results;
use whiskers_launcher_core::features::core::extensions::get_extension_request;

pub const ID: &str = "lighttigerxiv/bookmarks";

#[tokio::main]
async fn main() {
    let request = get_extension_request();

    match request.request_type {
        whiskers_launcher_core::features::extensions::ExtensionRequestType::GetResults => {
            on_get_results(request.clone());
        }
        whiskers_launcher_core::features::extensions::ExtensionRequestType::RunCommand => {
            on_run_commands(request.clone()).await
        }
    }
}
