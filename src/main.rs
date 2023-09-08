use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::thread::{sleep};
use std::time::Duration;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use simple_kl_rs::actions::{DialogAction, DialogField, ExtensionAction, get_dialog_result, get_dialog_results, InputField, notify, OpenInBrowser, ResultAction, SelectField, SelectOption};
use simple_kl_rs::extensions::{emit_results, Function, get_parameters};
use simple_kl_rs::paths::{get_extension_icon, get_home_path};
use simple_kl_rs::results::{IconWithTextResult, IconWithTitleAndDescriptionResult, SimpleKLResult};
use crate::bookmarks::{Bookmark, BookmarkGroup, BookmarksFile};

pub mod bookmarks;

fn get_bookmarks_folder_path() -> Option<PathBuf> {
    let mut path = get_home_path()
        .expect("Error getting home path");

    path.push(".config/simple-kl-bookmarks");

    return Some(path);
}

fn get_bookmarks_file_path() -> Option<PathBuf> {
    let mut path = get_home_path()
        .expect("Error getting home path");

    path.push(".config/simple-kl-bookmarks/bookmarks.yml");

    return Some(path);
}

fn init_bookmarks() {
    let bookmarks_folder = get_bookmarks_folder_path()
        .expect("Error getting bookmarks folder");

    //Creates the folder if it doesn't exist
    if !Path::new(&bookmarks_folder).exists() {
        fs::create_dir_all(bookmarks_folder)
            .expect("Error creating bookmarks folder");

        let mut file = File::create(get_bookmarks_file_path().unwrap())
            .expect("Error creating bookmarks file");

        file.flush()
            .expect("Error closing bookmarks file")
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
    let bookmarks_file = get_bookmarks_file_path()
        .expect("Error getting bookmarks file");

    let file_content = fs::read_to_string(&bookmarks_file)
        .expect("Error reading bookmarks file");

    return Some(serde_yaml::from_str(&file_content).unwrap());
}

fn get_bookmarks_select_options()-> Vec<SelectOption>{
    let bookmarks_file = get_bookmarks_file().unwrap();
    let mut bookmarks_options: Vec<SelectOption> = Vec::new();

    let mut bookmarks = bookmarks_file.to_owned().bookmarks;
    bookmarks.sort_by_key(|g| g.name.to_owned());

    for bookmark in bookmarks{

        bookmarks_options.push(SelectOption::new(
            &bookmark.id.to_string(),
            &bookmark.name,
        ))
    }

    return bookmarks_options;
}

fn get_bookmarks_select_options_default_value()-> String{
    let bookmarks_file = get_bookmarks_file().unwrap();

    let mut bookmarks = bookmarks_file.to_owned().bookmarks;
    bookmarks.sort_by_key(|g| g.name.to_owned());

    return bookmarks[0].id.to_string()
}

fn get_groups_select_options()-> Vec<SelectOption>{
    let bookmarks_file = get_bookmarks_file().unwrap();
    let mut groups_options: Vec<SelectOption> = Vec::new();

    let mut groups = bookmarks_file.to_owned().groups;
    groups.sort_by_key(|g| g.name.to_owned());

    for group in groups{

        groups_options.push(SelectOption::new(
            &group.id.to_string(),
            &group.name,
        ))
    }

    return groups_options;
}

fn get_groups_select_options_default_value()-> String{
    let bookmarks_file = get_bookmarks_file().unwrap();

    let mut groups = bookmarks_file.to_owned().groups;
    groups.sort_by_key(|g| g.name.to_owned());

    return groups[0].id.to_string()
}



fn main() {
    init_bookmarks();

    let parameters = get_parameters().unwrap();
    let function = parameters.function;
    let id = "com-lighttigerxiv-bookmarks";

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
                emit_results(results.to_owned());
            }

            let mut has_add_keyword = false;
            let mut has_remove_keyword = false;
            let mut has_edit_keyword = false;

            for (index, text) in splitted_search_text.iter().enumerate() {
                if index == 0 {
                    has_add_keyword = text.trim().to_lowercase() == "add";
                    has_remove_keyword = text.trim().to_lowercase() == "remove";
                    has_edit_keyword = text.trim().to_lowercase() == "edit";
                } else {
                    next_text = next_text + " " + text;
                }
            }

            if has_add_keyword {
                results.push(SimpleKLResult::IconWithText(IconWithTextResult::new_with_color(
                    get_extension_icon(id, "@src/images/plus.svg").unwrap(),
                    "accent",
                    "Add bookmark",
                    ResultAction::DialogAction(DialogAction::new()
                        .extension_id(id)
                        .title("Add Bookmark")
                        .action("add_bookmark")
                        .button_text("Add bookmark")
                        .fields(vec![
                            DialogField::Input(InputField::new()
                                .id("name")
                                .title("Name")
                                .description("The bookmark name")
                                .placeholder("Name")
                                .to_owned()
                            ),
                            DialogField::Input(InputField::new()
                                .id("url")
                                .title("Url")
                                .description("The bookmark url")
                                .placeholder("Url")
                                .to_owned()
                            ),
                        ])
                        .to_owned()
                    ),
                )));

                results.push(SimpleKLResult::IconWithText(IconWithTextResult::new_with_color(
                    get_extension_icon(id, "@src/images/plus.svg").unwrap(),
                    "accent",
                    "Add group",
                    ResultAction::DialogAction(DialogAction::new()
                        .extension_id(id)
                        .title("Add Group")
                        .action("add_group")
                        .button_text("Add group")
                        .fields(vec![
                            DialogField::Input(InputField::new()
                                .id("name")
                                .title("Name")
                                .description("The group name")
                                .placeholder("Name")
                                .to_owned()
                            ),
                        ])
                        .to_owned()
                    ),
                )));


                if groups.len() > 0 {

                    let groups_options = get_groups_select_options();
                    let groups_default_value = get_groups_select_options_default_value();
                    let bookmarks_options = get_bookmarks_select_options();
                    let bookmarks_default_value = get_bookmarks_select_options_default_value();

                    results.push(SimpleKLResult::IconWithText(IconWithTextResult::new_with_color(
                        get_extension_icon(id, "@src/images/plus.svg").unwrap(),
                        "accent",
                        "Add bookmark to group",
                        ResultAction::DialogAction(DialogAction::new()
                            .extension_id(id)
                            .title("Add Bookmark To Group")
                            .action("add_bookmark_to_group")
                            .button_text("Add bookmark")
                            .fields(vec![
                                DialogField::Select(SelectField::new(
                                    "group",
                                    &groups_default_value,
                                    "Group",
                                    "Select the group to add a bookmark",
                                    groups_options)
                                ),
                                DialogField::Select(SelectField::new(
                                    "bookmark",
                                    &bookmarks_default_value,
                                    "Bookmark",
                                    "Select the bookmark to add to a group",
                                    bookmarks_options)
                                ),
                            ])
                            .to_owned()
                        ),
                    )));
                }


                emit_results(results.to_owned());
            }


            if has_remove_keyword {

                if !&bookmarks.is_empty() {
                    let bookmarks_options = get_bookmarks_select_options();
                    let bookmarks_default_value = get_bookmarks_select_options_default_value();

                    results.push(SimpleKLResult::IconWithText(IconWithTextResult::new_with_color(
                        get_extension_icon(id, "@src/images/trash.svg").unwrap(),
                        "accent",
                        "Remove bookmark",
                        ResultAction::DialogAction(DialogAction::new()
                            .extension_id(id)
                            .title("Remove Bookmark")
                            .action("remove_bookmark")
                            .button_text("Remove bookmark")
                            .fields(vec![
                                DialogField::Select(SelectField::new(
                                    "bookmark",
                                    &bookmarks_default_value,
                                    "Remove Bookmark",
                                    "Select the bookmark to remove",
                                    bookmarks_options)
                                )
                            ])
                            .to_owned()
                        ),
                    )));
                }

                if !&groups.is_empty(){
                    let groups_options = get_groups_select_options();
                    let groups_default_value = get_groups_select_options_default_value();

                    results.push(SimpleKLResult::IconWithText(IconWithTextResult::new_with_color(
                        get_extension_icon(id, "@src/images/trash.svg").unwrap(),
                        "accent",
                        "Remove group",
                        ResultAction::DialogAction(DialogAction::new()
                            .extension_id(id)
                            .title("Remove Group")
                            .action("remove_group")
                            .button_text("Remove group")
                            .fields(vec![
                                DialogField::Select(SelectField::new(
                                    "group",
                                    &groups_default_value,
                                    "Remove Group",
                                    "Select the group to remove",
                                    groups_options)
                                )
                            ])
                            .to_owned()
                        ),
                    )));
                }

                emit_results(results.to_owned());
            }

            if has_edit_keyword{

                results.push(SimpleKLResult::IconWithText(IconWithTextResult::new_with_color(
                    get_extension_icon(id, "@src/images/pencil.svg").unwrap(),
                    "accent",
                    "Edit bookmarks",
                    ResultAction::ExtensionAction(ExtensionAction::new(id, "edit"))
                )));

                emit_results(results.to_owned());
            }

            for group in groups {
                if fuzzy_matcher.fuzzy_match(&group.name, &&search_text).is_some() {
                    results.push(SimpleKLResult::IconWithText(IconWithTextResult::new_with_color(
                        get_extension_icon(id, "@src/images/folder.svg").unwrap(),
                        "accent",
                        &format!("Open {} group", group.name),
                        ResultAction::ExtensionAction(ExtensionAction::new_with_args(
                            id,
                            "open_group",
                            vec![group.id.to_string()],
                        )),
                    )))
                }
            }

            for bookmark in bookmarks {

                let matches_name = fuzzy_matcher.fuzzy_match(&bookmark.name, &search_text).is_some();
                let matches_url = fuzzy_matcher.fuzzy_match(&bookmark.url, &search_text).is_some();

                if matches_name || matches_url {
                    results.push(SimpleKLResult::IconWithTitleAndDescription(IconWithTitleAndDescriptionResult::new_with_color(
                        get_extension_icon(id, "@src/images/bookmark.svg").unwrap(),
                        "accent",
                        &format!("Open {}", bookmark.name),
                        &bookmark.url,
                        ResultAction::OpenInBrowser(OpenInBrowser::new(&bookmark.url)),
                    )))
                }
            }

            emit_results(results.to_owned());
        }
        Function::RunAction => {
            let action = parameters.action.unwrap();

            if action == "add_bookmark" {
                let dialog_results = get_dialog_results().unwrap();
                let bookmark_name = &dialog_results.iter().find(|b| b.field_id == "name").unwrap().value;
                let bookmark_url = &dialog_results.iter().find(|b| b.field_id == "url").unwrap().value;


                let mut bookmarks: Vec<Bookmark> = bookmarks_file.to_owned().bookmarks;
                let groups: Vec<BookmarkGroup> = bookmarks_file.to_owned().groups;

                bookmarks.sort_by_key(|b| b.id);

                if bookmarks.is_empty() {
                    bookmarks.push(Bookmark::new(0, bookmark_name, bookmark_url));
                } else {
                    let last_id = bookmarks[bookmarks.len() - 1].id;

                    bookmarks.push(Bookmark::new(last_id + 1, bookmark_name, bookmark_url));
                }

                let new_bookmarks_file = BookmarksFile {
                    bookmarks,
                    groups,
                };

                let new_bookmarks_file_yaml = serde_yaml::to_string(&new_bookmarks_file).unwrap();

                fs::write(get_bookmarks_file_path().unwrap(), new_bookmarks_file_yaml)
                    .expect("Error writing bookmarks file");

                notify("Add Bookmark", &format!("{} added successfully", bookmark_name));
            }

            if action == "add_group" {
                let dialog_results = get_dialog_results().unwrap();
                let group_name = &dialog_results.iter().find(|b| b.field_id == "name").unwrap().value;

                let bookmarks = bookmarks_file.to_owned().bookmarks;
                let mut groups: Vec<BookmarkGroup> = bookmarks_file.to_owned().groups;

                groups.sort_by_key(|b| b.id);

                if groups.is_empty() {
                    groups.push(BookmarkGroup::new(0, group_name, vec![]));
                } else {
                    let last_id = groups[groups.len() - 1].id;

                    groups.push(BookmarkGroup::new(last_id + 1, group_name, vec![]));
                }

                let new_bookmarks_file = BookmarksFile {
                    bookmarks,
                    groups,
                };

                let new_bookmarks_file_yaml = serde_yaml::to_string(&new_bookmarks_file).unwrap();

                fs::write(get_bookmarks_file_path().unwrap(), new_bookmarks_file_yaml)
                    .expect("Error writing bookmarks file");

                notify("Add Group", &format!("{} added successfully", group_name));
            }

            if action == "add_bookmark_to_group" {
                let group_id_string = get_dialog_result("group").unwrap();
                let group_id: usize = group_id_string.parse().unwrap();
                let bookmark_id_string = get_dialog_result("bookmark").unwrap();
                let bookmark_id: usize = bookmark_id_string.parse().unwrap();

                let mut new_bookmarks_file = bookmarks_file.to_owned();

                let group_index = bookmarks_file.to_owned().groups.iter().position(|g| g.id == group_id).unwrap();
                let mut new_group_bookmarks = bookmarks_file.to_owned().groups[group_index].bookmarks.to_owned();

                new_group_bookmarks.push(bookmark_id);

                new_bookmarks_file.groups[group_index].bookmarks = new_group_bookmarks;

                let bookmarks_file_yaml = serde_yaml::to_string(&new_bookmarks_file).unwrap();

                fs::write(get_bookmarks_file_path().unwrap(), bookmarks_file_yaml)
                    .expect("Error writing bookmarks file");

                notify("Add bookmark to group", "Successfully added bookmark to group");
            }

            if action == "remove_bookmark" {
                let bookmark_id_string = get_dialog_result("bookmark").unwrap();
                let bookmark_id: usize = bookmark_id_string.parse().unwrap();

                let mut new_bookmarks = bookmarks_file.to_owned().bookmarks;
                let mut new_groups = bookmarks_file.to_owned().groups;

                new_bookmarks = new_bookmarks.iter().map(|b| b.to_owned())
                    .filter(|b| b.id != bookmark_id).collect();

                for (index, group) in bookmarks_file.to_owned().groups.iter().enumerate() {
                    if group.bookmarks.iter().any(|b| b == &bookmark_id) {
                        new_groups[index].bookmarks = group.bookmarks.iter().map(|b| b.to_owned())
                            .filter(|b| b != &bookmark_id).collect();
                    }
                }

                let new_bookmarks_file = BookmarksFile{
                    bookmarks: new_bookmarks,
                    groups: new_groups
                };

                let new_bookmarks_file_yaml = serde_yaml::to_string(&new_bookmarks_file).unwrap();

                fs::write(get_bookmarks_file_path().unwrap(), new_bookmarks_file_yaml)
                    .expect("Error writing bookmarks file");

                notify("Remove Bookmark", "Removed bookmark successfully");
            }

            if action == "remove_group"{
                let group_id_string = get_dialog_result("group").unwrap();
                let group_id: usize = group_id_string.parse().unwrap();

                let new_groups: Vec<BookmarkGroup> = bookmarks_file.to_owned().groups
                    .iter().map(|g|g.to_owned()).filter(|g|g.id != group_id).collect();

                let new_bookmarks_file = BookmarksFile{
                    bookmarks: bookmarks_file.to_owned().bookmarks,
                    groups: new_groups
                };

                let new_bookmarks_file_yaml = serde_yaml::to_string(&new_bookmarks_file).unwrap();

                fs::write(get_bookmarks_file_path().unwrap(), new_bookmarks_file_yaml).unwrap();

                notify("Remove Group", "Removed group successfully");
            }

            if action == "open_group" {
                let custom_args = parameters.custom_args.unwrap();
                let group_id_string = &custom_args[0];
                let group_id: usize = group_id_string.parse().unwrap();

                let group = bookmarks_file.to_owned().groups.iter().map(|g| g.to_owned()).find(|g| g.id == group_id).unwrap();
                let group_bookmarks = &group.bookmarks;
                let bookmarks = bookmarks_file.to_owned().bookmarks;

                for bookmark_id in group_bookmarks {
                    let bookmark = &bookmarks.iter().find(|b| b.id == bookmark_id.to_owned()).unwrap();
                    open::that(&bookmark.url).unwrap();
                    sleep(Duration::from_millis(1000));
                }
            }

            if action == "edit"{
                open::that(get_bookmarks_file_path().unwrap()).unwrap();
            }
        }
    }
}
