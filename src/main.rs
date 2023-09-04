use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{PathBuf};
use std::process::exit;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use simple_kl_rs::actions::{DoNothingAction, ExtensionAction, OpenInBrowser, ResultAction};
use simple_kl_rs::extensions::Function::{GetResults, RunAction};
use simple_kl_rs::extensions::{emit_results, get_parameters};
use simple_kl_rs::paths::{get_extension_icon, get_home_path};
use simple_kl_rs::results::{
    IconWithTextResult, IconWithTitleAndDescriptionResult, SimpleKLResult,
};

#[derive(serde::Serialize, serde::Deserialize)]
struct Bookmark {
    name: String,
    url: String,
}

fn get_bookmarks_folder_path() -> PathBuf {

    let mut path = get_home_path().unwrap();
    path.push(".config/simple-kl-bookmarks");

    return path
}

fn get_bookmarks_file_path() -> PathBuf {

    let mut path = get_home_path().unwrap();
    path.push(".config/simple-kl-bookmarks/bookmarks.yml");

    return path
}

fn get_bookmarks() -> Vec<Bookmark> {
    let mut bookmarks_file =
        File::open(&get_bookmarks_file_path()).expect("Error opening bookmarks file");
    let mut file_content = "".to_string();

    bookmarks_file
        .read_to_string(&mut file_content)
        .expect("Error reading bookmarks");

    bookmarks_file
        .flush()
        .expect("Error closing bookmarks file");

    if file_content.is_empty() {
        file_content = "[]".to_owned()
    }

    return serde_yaml::from_str(&file_content).expect("Error getting bookmarks from yaml");
}

fn main() {
    let parameters = get_parameters().unwrap();
    let function = parameters.to_owned().function;
    let id = "com-lighttigerxiv-bookmarks";

    // Init bookmarks
    if !&get_bookmarks_file_path().exists() {
        fs::create_dir_all(&get_bookmarks_folder_path()).expect("Error creating folder");
        File::create(&get_bookmarks_file_path()).expect("Error creating bookmarks file");
    }

    match function {
        GetResults => {
            let search_text = parameters.search_text.unwrap();
            let splitted_search_text: Vec<&str> = search_text.split_whitespace().collect();
            let matcher = SkimMatcherV2::default();
            let mut results: Vec<SimpleKLResult> = Vec::new();

            let contains_add_word = splitted_search_text.iter().any(|e| e == &"add");
            let contains_as_word = splitted_search_text.iter().any(|e| e == &"as");
            let contains_remove_word = splitted_search_text.iter().any(|e| e == &"remove");
            let contains_edit_word = splitted_search_text.iter().any(|e| e == &"edit");

            if search_text.trim().is_empty() {
                emit_results(results);
                exit(0);
            }

            let add_index = splitted_search_text
                .iter()
                .position(|e| e == &"add")
                .unwrap_or(9999);

            let as_index = splitted_search_text
                .iter()
                .position(|e| e == &"as")
                .unwrap_or(9999);

            let remove_index = splitted_search_text
                .iter()
                .position(|e| e == &"remove")
                .unwrap_or(9999);

            if contains_add_word && !contains_as_word {
                let url = match splitted_search_text.len() > add_index {
                    true => splitted_search_text[add_index + 1].to_string(),
                    false => "".to_string(),
                };

                results.push(SimpleKLResult::IconWithTitleAndDescription(
                    IconWithTitleAndDescriptionResult::new_with_color(
                        get_extension_icon(id, "@src/images/plus.svg").unwrap(),
                        "accent",
                        "Name:",
                        &format!("Url: {}", url),
                        ResultAction::DoNothingAction(DoNothingAction {}),
                    ),
                ));

                emit_results(results);
                exit(0);
            } else if contains_add_word && contains_as_word {
                let url = match splitted_search_text.len() > add_index {
                    true => splitted_search_text[add_index + 1].to_string(),
                    false => "".to_string(),
                };

                let name = match splitted_search_text.len() > add_index {
                    true => {
                        let mut name = "".to_string();

                        for (index, word) in splitted_search_text.iter().enumerate() {
                            if index > as_index {
                                name = name + word.to_owned() + " ";
                            }
                        }
                        name = name.trim_end().to_string();
                        name
                    }
                    false => "".to_string(),
                };

                results.push(SimpleKLResult::IconWithTitleAndDescription(
                    IconWithTitleAndDescriptionResult::new_with_color(
                        get_extension_icon(id, "@src/images/plus.svg").unwrap(),
                        "accent",
                        &format!("Name: {}", name),
                        &format!("Url: {}", url),
                        ResultAction::ExtensionAction(ExtensionAction::new_with_args(
                            id,
                            "add",
                            vec![url, name],
                        )),
                    ),
                ));

                emit_results(results);
                exit(0);
            } else if contains_remove_word {
                let search_text = match splitted_search_text.len() > remove_index {
                    true => {
                        let mut search_text = "".to_string();

                        for (index, word) in splitted_search_text.iter().enumerate() {
                            if index > remove_index {
                                search_text = search_text + word.to_owned() + " ";
                            }
                        }
                        search_text = search_text.trim_end().to_string();
                        search_text
                    }
                    false => "".to_string(),
                };

                if search_text.trim().is_empty() {
                    emit_results(results);
                    exit(0);
                }

                let bookmarks = get_bookmarks();

                for bookmark in bookmarks {
                    if bookmark.url.contains(&search_text)
                        || matcher.fuzzy_match(&bookmark.name, &search_text).is_some()
                    {
                        results.push(SimpleKLResult::IconWithTitleAndDescription(
                            IconWithTitleAndDescriptionResult::new_with_color(
                                get_extension_icon(id, "@src/images/trash.svg").unwrap(),
                                "accent",
                                &format!("Remove {}", bookmark.name),
                                &bookmark.url,
                                ResultAction::ExtensionAction(ExtensionAction::new_with_args(
                                    id,
                                    "remove",
                                    vec![bookmark.url.to_owned()],
                                )),
                            ),
                        ));
                    }
                }

                emit_results(results);
                exit(0);
            } else if contains_edit_word {
                results.push(SimpleKLResult::IconWithText(
                    IconWithTextResult::new_with_color(
                        get_extension_icon(id, "@src/images/pencil.svg").unwrap(),
                        "accent",
                        "Edit bookmarks",
                        ResultAction::ExtensionAction(ExtensionAction::new(
                            id,
                            "edit",
                        )),
                    ),
                ));

                emit_results(results);
                exit(0);
            }

            if search_text.trim().is_empty() {
                emit_results(results);
                exit(0);
            }

            let bookmarks = get_bookmarks();

            for bookmark in bookmarks {
                let matches_name = matcher.fuzzy_match(&bookmark.name, &search_text).is_some();
                let matches_url = matcher.fuzzy_match(&bookmark.url, &search_text).is_some();

                if matches_name || matches_url {
                    results.push(SimpleKLResult::IconWithTitleAndDescription(
                        IconWithTitleAndDescriptionResult::new_with_color(
                            get_extension_icon(
                                id,
                                "@src/images/bookmark.svg",
                            ).unwrap(),
                            "accent",
                            &bookmark.name,
                            &bookmark.url,
                            ResultAction::OpenInBrowser(OpenInBrowser { url: bookmark.url.to_owned() }),
                        ),
                    ))
                }
            }

            emit_results(results);
            exit(0);
        }
        RunAction => match parameters.action.unwrap().as_str() {
            "add" => {
                let args = parameters.custom_args.unwrap();
                let url = args[0].to_owned();
                let name = args[1].to_owned();
                let mut bookmarks = get_bookmarks();

                bookmarks.push(Bookmark { name, url });

                let bookmarks_yaml = serde_yaml::to_string(&bookmarks)
                    .expect("Error converting bookmarks to a yaml");

                let mut bookmarks_file =
                    File::create(&get_bookmarks_file_path()).expect("Error opening bookmarks file");

                bookmarks_file
                    .write_all(bookmarks_yaml.as_bytes())
                    .expect("Error writing bookmarks yaml");

                bookmarks_file
                    .flush()
                    .expect("Error closing bookmarks file");

                exit(0);
            }
            "remove" => {
                let bookmarks = get_bookmarks();
                let args = parameters.custom_args.unwrap();
                let url = args[0].to_owned();

                let new_bookmarks: Vec<&Bookmark> =
                    bookmarks.iter().filter(|e| e.url != url).collect();
                let new_bookmarks_yaml = serde_yaml::to_string(&new_bookmarks)
                    .expect("Error converting bookmarks to yaml");

                let mut bookmarks_file =
                    File::create(&get_bookmarks_file_path()).expect("Error opening bookmarks file");
                bookmarks_file
                    .write_all(&new_bookmarks_yaml.as_bytes())
                    .expect("Error writing in bookmarks file");
                bookmarks_file
                    .flush()
                    .expect("Error closing bookmarks file");
            }
            "edit" => {
                open::that(&get_bookmarks_file_path()).expect("Error opening bookmarks file");
            }
            _ => {}
        },
    }
}
