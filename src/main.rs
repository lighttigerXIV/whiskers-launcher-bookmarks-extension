//Add this to hide commands on windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::bookmarks::{Bookmark, BookmarkGroup, BookmarksFile};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use resources::{get_plus_icon, EXTENSION_ID, get_trash_icon, get_pencil_icon, get_folder_icon, get_bookmark_icon};
use simple_kl_rs::actions::{
    get_check_group_results, get_dialog_result, get_dialog_results, CheckGroup, CheckOption,
    DialogAction, DialogField, ExtensionAction, InputField, OpenInBrowser, ResultAction,
    SelectField, SelectOption,
};
use simple_kl_rs::extensions::{emit_results, get_parameters, Function};
use simple_kl_rs::others::notify;
use simple_kl_rs::paths::get_home_path;
use simple_kl_rs::results::{
    IconWithTextResult, IconWithTitleAndDescriptionResult, SimpleKLResult,
};
use std::{fs, env};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;

pub mod bookmarks;
pub mod resources;

fn get_bookmarks_folder_path() -> Option<PathBuf> {
    if cfg!(target_os = "linux") {
        let mut path = get_home_path().expect("Error getting home path");
        path.push(".config/simple-kl-bookmarks");

        return Some(path)
    }

    if cfg!(target_os = "windows") {
        let mut path = Path::new(&env::var("APPDATA").unwrap()).to_owned();
        path.push("simple-kl-bookmarks");

        return Some(path)
    }

    return None
}

fn get_bookmarks_file_path() -> Option<PathBuf> {
    let mut path = get_bookmarks_folder_path().expect("Error getting home path");

    path.push("bookmarks.yml");

    return Some(path);
}

fn init_bookmarks() {
    let bookmarks_folder = get_bookmarks_folder_path().expect("Error getting bookmarks folder");

    //Creates the folder if it doesn't exist
    if !Path::new(&bookmarks_folder).exists() {
        fs::create_dir_all(bookmarks_folder).expect("Error creating bookmarks folder");

        let mut file = File::create(get_bookmarks_file_path().unwrap())
            .expect("Error creating bookmarks file");

        file.flush().expect("Error closing bookmarks file")
    }

    let file_content = fs::read_to_string(&get_bookmarks_file_path().unwrap())
        .expect("Error reading bookmarks file");

    if file_content.is_empty() {
        let file = BookmarksFile {
            bookmarks: vec![],
            groups: vec![],
        };

        let content = serde_yaml::to_string(&file).unwrap();

        fs::write(&get_bookmarks_file_path().unwrap(), content)
            .expect("Error writing bookmarks file");
    }
}

fn get_bookmarks_file() -> Option<BookmarksFile> {
    let bookmarks_file = get_bookmarks_file_path().expect("Error getting bookmarks file");

    let file_content = fs::read_to_string(&bookmarks_file).expect("Error reading bookmarks file");

    return Some(serde_yaml::from_str(&file_content).unwrap());
}

fn get_bookmarks_select_options() -> Vec<SelectOption> {
    let bookmarks_file = get_bookmarks_file().unwrap();
    let mut bookmarks_options: Vec<SelectOption> = Vec::new();

    let mut bookmarks = bookmarks_file.to_owned().bookmarks;
    bookmarks.sort_by_key(|g| g.name.to_owned());

    for bookmark in bookmarks {
        bookmarks_options.push(SelectOption::new(&bookmark.id.to_string(), &bookmark.name))
    }

    return bookmarks_options;
}

fn get_bookmarks_select_options_default_value() -> String {
    let bookmarks_file = get_bookmarks_file().unwrap();

    let mut bookmarks = bookmarks_file.to_owned().bookmarks;
    bookmarks.sort_by_key(|g| g.name.to_owned());

    return bookmarks[0].id.to_string();
}

fn get_groups_select_options() -> Vec<SelectOption> {
    let bookmarks_file = get_bookmarks_file().unwrap();
    let mut groups_options: Vec<SelectOption> = Vec::new();

    let mut groups = bookmarks_file.to_owned().groups;
    groups.sort_by_key(|g| g.name.to_owned());

    for group in groups {
        groups_options.push(SelectOption::new(&group.id.to_string(), &group.name))
    }

    return groups_options;
}

fn get_groups_select_options_default_value() -> String {
    let bookmarks_file = get_bookmarks_file().unwrap();

    let mut groups = bookmarks_file.to_owned().groups;
    groups.sort_by_key(|g| g.name.to_owned());

    return groups[0].id.to_string();
}

fn main() {
    init_bookmarks();

    let parameters = get_parameters().unwrap();
    let function = parameters.function;

    let bookmarks_file = get_bookmarks_file().unwrap();

    match function {
        Function::GetResults => {
            let bookmarks = bookmarks_file.to_owned().bookmarks;
            let groups = bookmarks_file.to_owned().groups;

            let fuzzy_matcher = SkimMatcherV2::default();

            let mut results: Vec<SimpleKLResult> = Vec::new();
            let search_text = parameters.search_text.unwrap();
            let splitted_search_text: Vec<&str> = search_text.split(" ").collect();
            let mut next_text = String::from("");

            if search_text.is_empty() {
                emit_results(&results);
            }

            let mut has_add_keyword = false;
            let mut has_remove_keyword = false;
            let mut has_edit_keyword = false;

            for (index, text) in splitted_search_text.iter().enumerate() {
                if index == 0 {
                    has_add_keyword = text.trim().to_lowercase() == "add";
                    has_remove_keyword = text.trim().to_lowercase() == "remove";
                    has_edit_keyword = text.trim().to_lowercase() == "edit";
                } else if index == 0 && !has_add_keyword && !has_remove_keyword && !has_edit_keyword
                {
                    next_text = next_text + " " + text;
                } else {
                    next_text = next_text + " " + text;
                }
            }

            next_text = next_text.trim().to_owned();

            if has_add_keyword {
                results.push(SimpleKLResult::IconWithText(
                    IconWithTextResult::new_with_color(
                        get_plus_icon().unwrap(),
                        "Add Bookmark",
                        ResultAction::DialogAction(
                            DialogAction::new(EXTENSION_ID, "Add Bookmark", "add_bookmark")
                                .button_text("Add Bookmark")
                                .fields(vec![
                                    DialogField::Input(
                                        InputField::new()
                                            .id("name")
                                            .title("Name")
                                            .description("The bookmark name")
                                            .placeholder("Name")
                                            .to_owned(),
                                    ),
                                    DialogField::Input(
                                        InputField::new()
                                            .id("url")
                                            .title("Url")
                                            .description("The bookmark url")
                                            .placeholder("Url")
                                            .to_owned(),
                                    ),
                                ])
                                .to_owned(),
                        ),
                    ),
                ));

                results.push(SimpleKLResult::IconWithText(
                    IconWithTextResult::new_with_color(
                        get_plus_icon().unwrap(),
                        "Add Group",
                        ResultAction::DialogAction(
                            DialogAction::new(EXTENSION_ID, "Add Group", "add_group")
                                .button_text("Add Group")
                                .fields(vec![DialogField::Input(
                                    InputField::new()
                                        .id("name")
                                        .title("Name")
                                        .description("The group name")
                                        .placeholder("Name")
                                        .to_owned(),
                                )])
                                .to_owned(),
                        ),
                    ),
                ));

                emit_results(&results);
            }

            if has_remove_keyword {
                if !&bookmarks.is_empty() {
                    let bookmarks_options = get_bookmarks_select_options();
                    let bookmarks_default_value = get_bookmarks_select_options_default_value();

                    results.push(SimpleKLResult::IconWithText(
                        IconWithTextResult::new_with_color(
                            get_trash_icon().unwrap(),
                            "Remove Bookmark",
                            ResultAction::DialogAction(
                                DialogAction::new(EXTENSION_ID, "Remove Bookmark", "remove_bookmark")
                                    .button_text("Remove Bookmark")
                                    .fields(vec![DialogField::Select(SelectField::new(
                                        "bookmark",
                                        &bookmarks_default_value,
                                        "Remove Bookmark",
                                        "Select the bookmark to remove",
                                        bookmarks_options,
                                    ))])
                                    .to_owned(),
                            ),
                        ),
                    ));
                }

                if !&groups.is_empty() {
                    let groups_options = get_groups_select_options();
                    let groups_default_value = get_groups_select_options_default_value();

                    results.push(SimpleKLResult::IconWithText(
                        IconWithTextResult::new_with_color(
                            get_trash_icon().unwrap(),
                            "Remove Group",
                            ResultAction::DialogAction(
                                DialogAction::new(EXTENSION_ID, "Remove Group", "remove_group")
                                    .button_text("Remove Group")
                                    .fields(vec![DialogField::Select(SelectField::new(
                                        "group",
                                        &groups_default_value,
                                        "Remove Group",
                                        "Select the group to remove",
                                        groups_options,
                                    ))])
                                    .to_owned(),
                            ),
                        ),
                    ));
                }

                emit_results(&results);
            }

            if has_edit_keyword {
                if !groups.is_empty() && !next_text.is_empty() {
                    let mut sorted_bookmarks = bookmarks.to_owned();
                    sorted_bookmarks.sort_by_key(|b| b.name.to_owned());

                    for group in groups.to_owned() {
                        if fuzzy_matcher.fuzzy_match(&group.name, &next_text).is_some() {
                            let mut fields: Vec<DialogField> = vec![];
                            let mut check_options: Vec<CheckOption> = vec![];

                            for bookmark in sorted_bookmarks.to_owned() {
                                check_options.push(
                                    CheckOption::new(&bookmark.id.to_string(), &bookmark.name)
                                        .checked(group.bookmarks.contains(&bookmark.id))
                                        .description("This is a description")
                                        .to_owned(),
                                );
                            }

                            fields.push(DialogField::Input(
                                InputField::new()
                                    .id("name")
                                    .title("Name")
                                    .description("The group name")
                                    .value(&group.name)
                                    .to_owned(),
                            ));

                            fields.push(DialogField::CheckGroup(
                                CheckGroup::new(&format!("group-{}", group.id), "Bookmarks")
                                    .options(check_options)
                                    .to_owned(),
                            ));

                            results.push(SimpleKLResult::IconWithText(
                                IconWithTextResult::new_with_color(
                                    get_pencil_icon().unwrap(),
                                    &format!("Edit {} group", &group.name),
                                    ResultAction::DialogAction(
                                        DialogAction::new(
                                            EXTENSION_ID,
                                            &format!("Edit {}", &group.name),
                                            "edit_group",
                                        )
                                        .fields(fields)
                                        .to_owned(),
                                    ),
                                ),
                            ))
                        }
                    }
                }

                emit_results(&results);
            }

            for group in groups {
                if fuzzy_matcher
                    .fuzzy_match(&group.name, &search_text)
                    .is_some()
                {
                    results.push(SimpleKLResult::IconWithText(
                        IconWithTextResult::new_with_color(
                            get_folder_icon().unwrap(),
                            &format!("{} Group", group.name),
                            ResultAction::ExtensionAction(ExtensionAction::new_with_args(
                                EXTENSION_ID,
                                "open_group",
                                vec![group.id.to_string()],
                            )),
                        ),
                    ))
                }
            }

            for bookmark in bookmarks {
                let matches_name = fuzzy_matcher
                    .fuzzy_match(&bookmark.name, &search_text)
                    .is_some();
                let matches_url = fuzzy_matcher
                    .fuzzy_match(&bookmark.url, &search_text)
                    .is_some();

                if matches_name || matches_url {
                    results.push(SimpleKLResult::IconWithTitleAndDescription(
                        IconWithTitleAndDescriptionResult::new_with_color(
                            get_bookmark_icon().unwrap(),
                            "accent",
                            &bookmark.name,
                            &bookmark.url,
                            ResultAction::OpenInBrowser(OpenInBrowser::new(&bookmark.url)),
                        ),
                    ))
                }
            }

            emit_results(&results);
        }
        Function::RunAction => {
            let action = parameters.action.unwrap();

            if action == "add_bookmark" {
                let dialog_results = get_dialog_results().unwrap();
                let bookmark_name = &dialog_results
                    .iter()
                    .find(|b| b.field_id == "name")
                    .unwrap()
                    .value;
                let bookmark_url = &dialog_results
                    .iter()
                    .find(|b| b.field_id == "url")
                    .unwrap()
                    .value;

                let mut bookmarks: Vec<Bookmark> = bookmarks_file.to_owned().bookmarks;
                let groups: Vec<BookmarkGroup> = bookmarks_file.to_owned().groups;

                bookmarks.sort_by_key(|b| b.id);

                if bookmarks.is_empty() {
                    bookmarks.push(Bookmark::new(0, bookmark_name, bookmark_url));
                } else {
                    let last_id = bookmarks[bookmarks.len() - 1].id;

                    bookmarks.push(Bookmark::new(last_id + 1, bookmark_name, bookmark_url));
                }

                let new_bookmarks_file = BookmarksFile { bookmarks, groups };

                let new_bookmarks_file_yaml = serde_yaml::to_string(&new_bookmarks_file).unwrap();

                fs::write(get_bookmarks_file_path().unwrap(), new_bookmarks_file_yaml)
                    .expect("Error writing bookmarks file");

                notify(
                    "Add Bookmark",
                    &format!("{} added successfully", bookmark_name),
                );
            }

            if action == "add_group" {
                let dialog_results = get_dialog_results().unwrap();
                let group_name = &dialog_results
                    .iter()
                    .find(|b| b.field_id == "name")
                    .unwrap()
                    .value;

                let bookmarks = bookmarks_file.to_owned().bookmarks;
                let mut groups: Vec<BookmarkGroup> = bookmarks_file.to_owned().groups;

                groups.sort_by_key(|b| b.id);

                if groups.is_empty() {
                    groups.push(BookmarkGroup::new(0, group_name, vec![]));
                } else {
                    let last_id = groups[groups.len() - 1].id;

                    groups.push(BookmarkGroup::new(last_id + 1, group_name, vec![]));
                }

                let new_bookmarks_file = BookmarksFile { bookmarks, groups };

                let new_bookmarks_file_yaml = serde_yaml::to_string(&new_bookmarks_file).unwrap();

                fs::write(get_bookmarks_file_path().unwrap(), new_bookmarks_file_yaml)
                    .expect("Error writing bookmarks file");

                notify("Add Group", &format!("{} added successfully", group_name));
            }

            if action == "remove_bookmark" {
                let bookmark_id_string = get_dialog_result("bookmark").unwrap();
                let bookmark_id: usize = bookmark_id_string.value.parse().unwrap();

                let mut new_bookmarks = bookmarks_file.to_owned().bookmarks;
                let mut new_groups = bookmarks_file.to_owned().groups;

                new_bookmarks = new_bookmarks
                    .iter()
                    .map(|b| b.to_owned())
                    .filter(|b| b.id != bookmark_id)
                    .collect();

                for (index, group) in bookmarks_file.to_owned().groups.iter().enumerate() {
                    if group.bookmarks.iter().any(|b| b == &bookmark_id) {
                        new_groups[index].bookmarks = group
                            .bookmarks
                            .iter()
                            .map(|b| b.to_owned())
                            .filter(|b| b != &bookmark_id)
                            .collect();
                    }
                }

                let new_bookmarks_file = BookmarksFile {
                    bookmarks: new_bookmarks,
                    groups: new_groups,
                };

                let new_bookmarks_file_yaml = serde_yaml::to_string(&new_bookmarks_file).unwrap();

                fs::write(get_bookmarks_file_path().unwrap(), new_bookmarks_file_yaml)
                    .expect("Error writing bookmarks file");

                notify("Remove Bookmark", "Removed bookmark successfully");
            }

            if action == "remove_group" {
                let group_id_string = get_dialog_result("group").unwrap();
                let group_id: usize = group_id_string.value.parse().unwrap();

                let new_groups: Vec<BookmarkGroup> = bookmarks_file
                    .to_owned()
                    .groups
                    .iter()
                    .map(|g| g.to_owned())
                    .filter(|g| g.id != group_id)
                    .collect();

                let new_bookmarks_file = BookmarksFile {
                    bookmarks: bookmarks_file.to_owned().bookmarks,
                    groups: new_groups,
                };

                let new_bookmarks_file_yaml = serde_yaml::to_string(&new_bookmarks_file).unwrap();

                fs::write(get_bookmarks_file_path().unwrap(), new_bookmarks_file_yaml).unwrap();

                notify("Remove Group", "Removed group successfully");
            }

            if action == "open_group" {
                let custom_args = parameters.custom_args.unwrap();
                let group_id_string = &custom_args[0];
                let group_id: usize = group_id_string.parse().unwrap();

                let group = bookmarks_file
                    .to_owned()
                    .groups
                    .iter()
                    .map(|g| g.to_owned())
                    .find(|g| g.id == group_id)
                    .unwrap();
                let group_bookmarks = &group.bookmarks;
                let bookmarks = bookmarks_file.to_owned().bookmarks;

                for bookmark_id in group_bookmarks {
                    let bookmark = &bookmarks
                        .iter()
                        .find(|b| b.id == bookmark_id.to_owned())
                        .unwrap();
                    open::that(&bookmark.url).unwrap();
                    sleep(Duration::from_millis(1000));
                }
            }

            if action == "edit_group" {
                let dialog_results = get_dialog_results().unwrap();
                let new_name = get_dialog_result("name").unwrap().value;
                let field_id = dialog_results[1].to_owned().field_id;
                let group_id: usize = field_id.replace("group-", "").parse().unwrap();

                let check_results = get_check_group_results(&field_id).unwrap();
                let mut selected_bookmarks: Vec<usize> = vec![];

                for result in check_results {
                    if result.checked {
                        let id: usize = result.id.parse().unwrap();
                        selected_bookmarks.push(id);
                    }
                }

                let mut new_groups: Vec<BookmarkGroup> = bookmarks_file.to_owned().groups;

                for (index, group) in bookmarks_file.to_owned().groups.iter().enumerate() {
                    if group.id == group_id {
                        new_groups[index].bookmarks = selected_bookmarks.to_owned();
                        new_groups[index].name = new_name.to_owned();
                    }
                }

                let new_bookmarks_file = BookmarksFile {
                    bookmarks: bookmarks_file.to_owned().bookmarks,
                    groups: new_groups,
                };

                let new_bookmarks_file_yaml = serde_yaml::to_string(&new_bookmarks_file).unwrap();

                fs::write(get_bookmarks_file_path().unwrap(), new_bookmarks_file_yaml)
                    .expect("Error wrting bookmarks file");

                notify("Edit Group", &format!("Group edited successfully"));
            }

            if action == "edit" {
                open::that(get_bookmarks_file_path().unwrap()).unwrap();
            }
        }
    }
}
