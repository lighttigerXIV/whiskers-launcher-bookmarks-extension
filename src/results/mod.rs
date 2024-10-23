use std::path::PathBuf;

use sniffer_rs::sniffer::Sniffer;
use whiskers_launcher_core::{
    features::extensions::{get_extension_setting, send_search_results, ExtensionRequest},
    results::{
        CopyTextAction, FormField, FormFilePickerField, FormInputField, FormToggleField,
        OpenFormAction, OpenLinkAction, ResultAction, RunExtensionAction, SearchResult,
        SearchResults,
    },
    utils::get_search_query,
};

use crate::{icons::get_icon_path, settings::functions::get_settings, ID};

pub fn on_get_results(request: ExtensionRequest) {
    let search_text = request.search_text.unwrap();
    let search = get_search_query(&search_text);

    if search_text.trim().is_empty() {
        show_default_results();
    }

    if let Some(keyword) = search.keyword {
        match keyword.as_str() {
            "e" => show_edit_results(&search.search_text),
            "edit" => show_edit_results(&search.search_text),
            "d" => show_delete_results(&search.search_text),
            "delete" => show_delete_results(&search.search_text),
            _ => show_results(&search_text),
        }
    }

    show_results(&search_text);
}

fn show_default_results() {
    let mut results = Vec::<SearchResult>::new();
    let bookmark_fields = vec![
        FormField::new_input_field(
            "name",
            FormInputField::new("Name", "The name of the bookmark")
                .set_placeholder("Type the bookmark name")
                .set_not_empty_validation(),
        ),
        FormField::new_input_field(
            "url",
            FormInputField::new("Url", "The url of the bookmark")
                .set_placeholder("Type the bookmark url")
                .set_not_empty_validation(),
        ),
        FormField::new_toggle_field(
            "use-icon",
            FormToggleField::new(
                "Icon",
                "Use the website icon instead of the default bookmark one",
                true,
            ),
        ),
    ];

    let mut group_fields = vec![
        FormField::new_input_field(
            "name",
            FormInputField::new("Name", "The name of the bookmark")
                .set_placeholder("Type the bookmark name")
                .set_not_empty_validation(),
        ),
        FormField::new_file_picker_field(
            "icon-path",
            FormFilePickerField::new("Icon (Optional)", "Select a icon for the group")
                .set_image_file_types(),
        ),
        FormField::new_toggle_field(
            "tint-icon",
            FormToggleField::new("Tint icon", "Tint the group custom icon", false),
        ),
    ];

    for bookmark in get_settings().bookmarks {
        group_fields.push(FormField::new_toggle_field(
            &bookmark.id.to_string(),
            FormToggleField::new(
                &bookmark.name,
                "Toggle to add this bookmark to the group",
                false,
            ),
        ));
    }

    results.push(
        SearchResult::new(
            "Create Bookmark",
            ResultAction::new_open_form_action(
                OpenFormAction::new(ID, "create-bookmark", bookmark_fields)
                    .set_title("Create Bookmark")
                    .set_action_text("Create"),
            ),
        )
        .set_icon(get_icon_path("plus"))
        .set_accent_icon_tint(),
    );

    results.push(
        SearchResult::new(
            "Create Group",
            ResultAction::new_open_form_action(
                OpenFormAction::new(ID, "create-group", group_fields)
                    .set_title("Create Group")
                    .set_action_text("Create"),
            ),
        )
        .set_icon(get_icon_path("plus"))
        .set_accent_icon_tint(),
    );

    send_search_results(SearchResults::new_list_results(results));
}

fn show_edit_results(search_text: &str) {
    let mut results = Vec::<SearchResult>::new();
    let settings = get_settings();
    let sniffer = Sniffer::new();

    for group in settings.to_owned().groups {
        if sniffer.matches(&group.name, search_text) {
            let bookmarks_ids = group.to_owned().bookmarks_ids;

            let name_field = FormField::new_input_field(
                "name",
                FormInputField::new("Name", "The name of the bookmark")
                    .set_text(&group.name)
                    .set_placeholder("Type the bookmark name")
                    .set_not_empty_validation(),
            );

            let mut icon_picker_field =
                FormFilePickerField::new("Icon (Optional)", "Select a icon for the group")
                    .set_image_file_types();

            if let Some(path) = group.to_owned().icon_path {
                icon_picker_field = icon_picker_field.set_file_path(path);
            }

            let icon_field = FormField::new_file_picker_field("icon-path", icon_picker_field);

            let tint_icon_field = FormField::new_toggle_field(
                "tint-icon",
                FormToggleField::new("Tint icon", "Tint the group custom icon", group.tint_icon),
            );

            let mut fields = vec![name_field, icon_field, tint_icon_field];

            for bookmark in settings.to_owned().bookmarks {
                let field = FormField::new_toggle_field(
                    &bookmark.id.to_string(),
                    FormToggleField::new(
                        &bookmark.name,
                        "Toggle to add this bookmark to the group",
                        bookmarks_ids.contains(&bookmark.id),
                    ),
                );

                fields.push(field);
            }

            let mut edit_group_result = SearchResult::new(
                format!("Edit Group || {}", &group.name),
                ResultAction::new_open_form_action(
                    OpenFormAction::new(ID, "edit-group", fields)
                        .set_title("Edit Group")
                        .set_action_text("Save")
                        .add_arg(group.id.to_string()),
                ),
            )
            .set_icon(if let Some(path) = group.to_owned().icon_path {
                PathBuf::from(path)
            } else {
                get_icon_path("pencil")
            });

            if group.icon_path.is_some() {
                if group.tint_icon {
                    edit_group_result = edit_group_result.set_accent_icon_tint();
                }
            } else {
                edit_group_result = edit_group_result.set_accent_icon_tint();
            }

            results.push(edit_group_result);
        }
    }

    for bookmark in settings.bookmarks {
        if sniffer.matches(&bookmark.name, search_text) {
            let name_field = FormField::new_input_field(
                "name",
                FormInputField::new("Name", "The name of the bookmark")
                    .set_text(&bookmark.name)
                    .set_placeholder("Type the bookmark name")
                    .set_not_empty_validation(),
            );

            let url_field = FormField::new_input_field(
                "url",
                FormInputField::new("Url", "The url of the bookmark")
                    .set_text(&bookmark.url)
                    .set_placeholder("Type the bookmark url")
                    .set_not_empty_validation(),
            );

            let use_icon_field = FormField::new_toggle_field(
                "use-icon",
                FormToggleField::new(
                    "Icon",
                    "Uses the website icon instead of the default bookmark one",
                    if bookmark.to_owned().icon_path.is_some() {
                        true
                    } else {
                        false
                    },
                ),
            );

            let mut edit_bookmark_result = SearchResult::new(
                format!("Edit Bookmark || {}", &bookmark.name),
                ResultAction::new_open_form_action(
                    OpenFormAction::new(
                        ID,
                        "edit-bookmark",
                        vec![name_field, url_field, use_icon_field],
                    )
                    .set_title("Edit Bookmark")
                    .set_action_text("Save")
                    .add_arg(bookmark.id.to_string()),
                ),
            )
            .set_icon(if let Some(path) = bookmark.to_owned().icon_path {
                PathBuf::from(path)
            } else {
                get_icon_path("pencil")
            });

            if bookmark.icon_path.is_none(){
                edit_bookmark_result = edit_bookmark_result.set_accent_icon_tint();
            }

            results.push(edit_bookmark_result);
        }
    }

    send_search_results(SearchResults::new_list_results(results));
}

fn show_delete_results(search_text: &str) {
    let mut results = Vec::<SearchResult>::new();
    let settings = get_settings();
    let sniffer = Sniffer::new();

    for group in settings.groups {
        if sniffer.matches(&group.name, search_text) {
            let mut result = SearchResult::new(
                format!("Delete Group | {}", group.name),
                ResultAction::new_run_extension_action(
                    RunExtensionAction::new(ID, "delete-group").add_arg(group.id.to_string()),
                )
                .set_dangerous(true),
            );

            if let Some(icon_path) = group.icon_path {
                result = result.set_icon(PathBuf::from(icon_path));

                if group.tint_icon {
                    result = result.set_accent_icon_tint();
                }
            } else {
                result = result
                    .set_icon(get_icon_path("trash"))
                    .set_accent_icon_tint();
            }

            results.push(result);
        }
    }

    for bookmark in settings.bookmarks {
        if sniffer.matches(&bookmark.name, search_text) {
            let mut result = SearchResult::new(
                format!("Delete Bookmark | {}", bookmark.name),
                ResultAction::new_run_extension_action(
                    RunExtensionAction::new(ID, "delete-bookmark").add_arg(bookmark.id.to_string()),
                )
                .set_dangerous(true),
            );

            if let Some(icon_path) = bookmark.icon_path {
                result = result.set_icon(PathBuf::from(icon_path));
            } else {
                result = result
                    .set_icon(get_icon_path("trash"))
                    .set_accent_icon_tint();
            }

            results.push(result);
        }
    }

    send_search_results(SearchResults::new_list_results(results));
}

fn show_results(search_text: &str) {
    let settings = get_settings();
    let mut results = Vec::<SearchResult>::new();
    let copy_url: bool = get_extension_setting(ID, "copy-url").unwrap() == "true";
    let sniffer = Sniffer::new();

    if !copy_url {
        for group in settings.groups {
            if sniffer.matches(&group.name, search_text) {
                let mut result = SearchResult::new(
                    &group.name,
                    ResultAction::new_run_extension_action(
                        RunExtensionAction::new(ID, "open-group").add_arg(&group.id.to_string()),
                    ),
                );

                if let Some(icon_path) = group.icon_path {
                    result = result.set_icon(PathBuf::from(icon_path));

                    if group.tint_icon {
                        result = result.set_accent_icon_tint();
                    }
                } else {
                    result = result
                        .set_icon(get_icon_path("folder"))
                        .set_accent_icon_tint();
                }

                results.push(result);
            }
        }
    }

    for bookmark in settings.bookmarks {
        if sniffer.matches(&bookmark.name, search_text) {
            let mut result = match copy_url {
                true => SearchResult::new(
                    &bookmark.name,
                    ResultAction::new_copy_text_action(CopyTextAction::new(&bookmark.url)),
                ),
                false => SearchResult::new(
                    &bookmark.name,
                    ResultAction::new_open_link_action(OpenLinkAction::new(&bookmark.url)),
                ),
            };

            if let Some(icon_path) = bookmark.icon_path {
                result = result.set_icon(PathBuf::from(icon_path));
            } else {
                result = result
                    .set_icon(get_icon_path("bookmark"))
                    .set_accent_icon_tint();
            }

            results.push(result);
        }
    }

    send_search_results(SearchResults::new_list_results(results));
}
