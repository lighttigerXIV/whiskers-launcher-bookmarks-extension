use std::{fs, io::Cursor};

use image::{io::Reader, ImageFormat};
use reqwest::Client;
use whiskers_launcher_rs::{
    api::extensions::{get_dialog_response, get_extension_request, ExtensionRequest},
    utils::send_notification,
};

use crate::{
    paths::get_favicons_dir,
    settings::{
        functions::{get_settings, write_settings},
        Bookmark, Group,
    },
};

pub async fn handle_actions(request: ExtensionRequest) {
    let action = request.to_owned().extension_action.unwrap();

    match action.as_str() {
        "create-bookmark" => create_bookmark().await,
        "create-group" => create_group(),
        "edit-bookmark" => edit_bookmark().await,
        "edit-group" => edit_group(),
        "open-group" => open_group().await,
        "delete-bookmark" => delete_bookmark(),
        "delete-group" => delete_group(),
        _ => {}
    }
}

async fn create_bookmark() {
    let response = get_dialog_response();
    let name = response.to_owned().get_result_value("name").unwrap();
    let url = response.to_owned().get_result_value("url").unwrap();

    let mut settings = get_settings();
    let mut bookmark = Bookmark::new(&name, &url);

    if response.to_owned().get_result_value("use-icon").unwrap() == "true" {
        let url = format!("https://www.google.com/s2/favicons?domain={}&sz=256", &url);

        let request = Client::new().get(&url).send().await;

        match request {
            Ok(response) => {
                if response.status().is_success() {
                    let bytes = response.bytes().await.unwrap();

                    let mut path = get_favicons_dir();

                    if !path.exists() {
                        fs::create_dir_all(&path).expect("Error creating directory");
                    }

                    path.push(format!("{}.png", bookmark.id));

                    let image = Reader::new(Cursor::new(&bytes))
                        .with_guessed_format()
                        .unwrap()
                        .decode()
                        .unwrap();

                    image
                        .save_with_format(&path, ImageFormat::Png)
                        .expect("Error saving image");

                    bookmark.icon_path(&path.into_os_string().into_string().unwrap());
                }
            }
            Err(_) => {
                send_notification(
                    "Error",
                    "Error getting icon. Make sure you have a valid url and internet connection",
                );
            }
        }
    }

    settings.bookmarks.push(bookmark);

    write_settings(settings);

    send_notification("Create bookmark", "Bookmark created successfully");
}

fn create_group() {
    let mut settings = get_settings();
    let response = get_dialog_response();
    let name = response.to_owned().get_result_value("name").unwrap();
    let icon_path = response.to_owned().get_result_value("icon-path").unwrap();
    let tint_icon = response.to_owned().get_result_value("tint-icon").unwrap();
    let results = response.results;
    let mut bookmarks_ids = Vec::<usize>::new();

    for result in results {
        if result.field_value == "true" {
            bookmarks_ids.push(result.field_id.parse().unwrap());
        }
    }

    let mut group = Group::new(name, bookmarks_ids).tint_icon(tint_icon == "true");

    if !icon_path.is_empty() {
        group.icon_path(icon_path);
    }

    settings.groups.push(group);

    write_settings(settings);
}

fn edit_group() {}

async fn edit_bookmark() {
    let response = get_dialog_response();
    let name = response.to_owned().get_result_value("name").unwrap();
    let url = response.to_owned().get_result_value("url").unwrap();
    let use_icon = response.to_owned().get_result_value("use-icon").unwrap();
    let bookmark_id: usize = response.to_owned().args.unwrap()[0].parse().unwrap();

    let mut settings = get_settings();
    let mut bookmarks = Vec::<Bookmark>::new();

    for bookmark in settings.bookmarks {
        if bookmark.id == bookmark_id {
            let mut path = get_favicons_dir();

            if use_icon == "true" {
                let url = format!("https://www.google.com/s2/favicons?domain={}&sz=256", &url);

                let request = Client::new().get(&url).send().await;

                match request {
                    Ok(response) => {
                        if response.status().is_success() {
                            let bytes = response.bytes().await.unwrap();

                            if !path.exists() {
                                fs::create_dir_all(&path).expect("Error creating directory");
                            }

                            path.push(format!("{}.png", bookmark.id));

                            let image = Reader::new(Cursor::new(&bytes))
                                .with_guessed_format()
                                .unwrap()
                                .decode()
                                .unwrap();

                            image
                                .save_with_format(&path, ImageFormat::Png)
                                .expect("Error saving image");
                        }
                    }
                    Err(_) => {
                        send_notification(
                            "Error",
                            "Error getting icon. Make sure you have a valid url and internet connection");
                    }
                }
            }

            bookmarks.push(Bookmark {
                id: bookmark_id.to_owned(),
                icon_path: if use_icon == "true" {
                    Some(path.into_os_string().into_string().unwrap())
                } else {
                    None
                },
                name: name.to_owned(),
                url: url.to_owned(),
            });
        } else {
            bookmarks.push(bookmark.to_owned());
        }
    }

    settings.bookmarks = bookmarks;
    write_settings(settings);
}

async fn open_group() {
    let response = get_extension_request();
    let args = response.args.expect("Expected argument with group id");
    let group_id: usize = args
        .get(0)
        .expect("Expected group id")
        .parse()
        .expect("Error parsing group id");

    let group = get_settings()
        .groups
        .iter()
        .find(|group| group.id == group_id)
        .expect("Expected group id")
        .to_owned();

    let bookmarks = get_settings().bookmarks;

    for bookmark_id in group.bookmarks_ids {
        if let Some(bookmark) = bookmarks.iter().find(|b| b.id == bookmark_id) {
            let bookmark = bookmark.to_owned();

            std::thread::spawn(move || {
                open::that(&bookmark.url).expect("Error opening bookmark");
            });
        }
    }
}

fn delete_bookmark() {
    let request = get_extension_request();
    let args = request.args.expect("Expected argument with bookmark");
    let bookmark_id: usize = args
        .get(0)
        .expect("Expected bookmark id")
        .parse()
        .expect("Error parsing bookmark id");

    let new_bookmarks: Vec<Bookmark> = get_settings()
        .bookmarks
        .iter()
        .map(|b| b.to_owned())
        .filter(|b| b.id != bookmark_id)
        .collect();

    let mut settings = get_settings();
    settings.bookmarks = new_bookmarks;

    write_settings(settings)
}

fn delete_group() {
    let request = get_extension_request();
    let args = request.args.expect("Expected argument with group");
    let group_id: usize = args
        .get(0)
        .expect("Expected group id")
        .parse()
        .expect("Error parsing group id");

    let new_groups: Vec<Group> = get_settings()
        .groups
        .iter()
        .map(|g| g.to_owned())
        .filter(|g| g.id != group_id)
        .collect();

    let mut settings = get_settings();
    settings.groups = new_groups;

    write_settings(settings)
}
