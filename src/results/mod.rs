use whiskers_launcher_rs::{
    action::{
        Action, CopyAction, DialogAction, ExtensionAction, Field, FileFilter, FilePickerField,
        InputField, OpenURLAction, ToggleField,
    },
    api::extensions::{get_extension_setting, send_response, ExtensionRequest},
    result::{TextResult, WLResult},
    utils::{fuzzy_matches, get_search},
};

use crate::{icons::get_icon_path, settings::functions::get_settings, EXTENSION_ID};

pub fn handle_results(request: ExtensionRequest) {
    let search_text = request.search_text.unwrap();
    let search = get_search(&search_text);

    if search_text.is_empty() {
        show_default_results();
    }

    if let Some(keyword) = search.keyword {
        match keyword.as_str() {
            "e" => show_edit_results(&search.search_text),
            "edit" => show_edit_results(&search.search_text),
            _ => show_search_results(&search_text),
        }
    }

    show_search_results(&search_text);
}

fn show_default_results() {
    let mut results = Vec::<WLResult>::new();
    let bookmark_fields = vec![
        Field::new_input(
            "name",
            InputField::new("", "Name", "The name of the bookmark")
                .placeholder("Type the bookmark name"),
        ),
        Field::new_input(
            "url",
            InputField::new("", "Url", "The url of the bookmark")
                .placeholder("Type the bookmark url"),
        ),
        Field::new_toggle(
            "use-icon",
            ToggleField::new(
                true,
                "Icon",
                "Uses the website icon instead of the default bookmark one",
            ),
        ),
    ];

    let mut group_fields = vec![
        Field::new_input(
            "name",
            InputField::new("", "Name", "The name of the bookmark")
                .placeholder("Type the bookmark name"),
        ),
        Field::new_file_picker(
            "icon-path",
            FilePickerField::new("Icon (Optional)", "Select a icon for the group").filters(vec![
                FileFilter::new(
                    "Images",
                    vec![
                        "png".to_string(),
                        "jpg".to_string(),
                        "jpeg".to_string(),
                        "svg".to_string(),
                    ],
                ),
            ]),
        ),
        Field::new_toggle(
            "tint-icon",
            ToggleField::new(false, "Tint icon", "Tint the group custom icon"),
        ),
    ];

    for bookmark in get_settings().bookmarks {
        group_fields.push(Field::new_toggle(
            &bookmark.id.to_string(),
            ToggleField::new(
                false,
                &bookmark.name,
                "Toggle to add this bookmark to the group",
            ),
        ));
    }

    results.push(WLResult::new_text(
        TextResult::new(
            "Create Bookmark",
            Action::new_dialog(DialogAction::new(
                EXTENSION_ID,
                "create-bookmark",
                "Create a bookmark",
                "Create",
                bookmark_fields,
            )),
        )
        .icon(get_icon_path("plus"))
        .tint("accent"),
    ));

    results.push(WLResult::new_text(
        TextResult::new(
            "Create Group",
            Action::new_dialog(DialogAction::new(
                EXTENSION_ID,
                "create-group",
                "Create a group",
                "Create",
                group_fields,
            )),
        )
        .icon(get_icon_path("plus"))
        .tint("accent"),
    ));

    send_response(results);
}

fn show_edit_results(search_text: &str) {
    let mut results = Vec::<WLResult>::new();
    let settings = get_settings();

    for group in settings.to_owned().groups {
        if fuzzy_matches(&group.name, search_text) {
            let bookmarks_ids = group.to_owned().bookmarks_ids;
            let mut fields = vec![
                Field::new_input(
                    "name",
                    InputField::new(&group.name, "Name", "The name of the bookmark")
                        .placeholder("Type the bookmark name"),
                ),
                Field::new_file_picker(
                    "icon-path",
                    FilePickerField::new("Icon (Optional)", "Select a icon for the group")
                        .filters(vec![FileFilter::new(
                            "Images",
                            vec![
                                "png".to_string(),
                                "jpg".to_string(),
                                "jpeg".to_string(),
                                "svg".to_string(),
                            ],
                        )])
                        .default_path(if let Some(path) = group.to_owned().icon_path {
                            path
                        } else {
                            "".to_string()
                        }),
                ),
                Field::new_toggle(
                    "tint-icon",
                    ToggleField::new(group.tint_icon, "Tint icon", "Tint the group custom icon"),
                ),
            ];

            for bookmark in settings.to_owned().bookmarks {
                fields.push(Field::new_toggle(
                    &bookmark.id.to_string(),
                    ToggleField::new(
                        bookmarks_ids.contains(&bookmark.id),
                        &bookmark.name,
                        "Toggle to add this bookmark to the group",
                    ),
                ));
            }

            results.push(WLResult::new_text(
                TextResult::new(
                    format!("Edit Group || {}", &group.name),
                    Action::new_dialog(
                        DialogAction::new(EXTENSION_ID, "edit-group", "Edit Group", "Save", fields)
                            .args(vec![group.id.to_string()]),
                    ),
                )
                .icon(if let Some(path) = group.to_owned().icon_path {
                    path
                } else {
                    get_icon_path("pencil")
                })
                .tint(if group.to_owned().icon_path.is_some() {
                    if group.to_owned().tint_icon {
                        "accent"
                    } else {
                        ""
                    }
                } else {
                    "accent"
                }),
            ));
        }
    }

    for bookmark in settings.bookmarks {
        if fuzzy_matches(&bookmark.name, search_text) {
            results.push(WLResult::new_text(
                TextResult::new(
                    format!("Edit Bookmark || {}", &bookmark.name),
                    Action::new_dialog(
                        DialogAction::new(
                            EXTENSION_ID,
                            "edit-bookmark",
                            "Edit Bookmark",
                            "Save",
                            vec![
                                Field::new_input(
                                    "name",
                                    InputField::new(
                                        &bookmark.name,
                                        "Name",
                                        "The name of the bookmark",
                                    )
                                    .placeholder("Type the bookmark name"),
                                ),
                                Field::new_input(
                                    "url",
                                    InputField::new(
                                        &bookmark.url,
                                        "Url",
                                        "The url of the bookmark",
                                    )
                                    .placeholder("Type the bookmark url"),
                                ),
                                Field::new_toggle(
                                    "use-icon",
                                    ToggleField::new(
                                        if bookmark.to_owned().icon_path.is_some() {
                                            true
                                        } else {
                                            false
                                        },
                                        "Icon",
                                        "Uses the website icon instead of the default bookmark one",
                                    ),
                                ),
                            ],
                        )
                        .args(vec![bookmark.id.to_string()]),
                    ),
                )
                .icon(if let Some(path) = bookmark.to_owned().icon_path {
                    path
                } else {
                    get_icon_path("pencil")
                })
                .tint(if bookmark.to_owned().icon_path.is_some() {
                    ""
                } else {
                    "accent"
                }),
            ));
        }
    }

    send_response(results);
}

fn show_search_results(search_text: &str) {
    let settings = get_settings();
    let mut results = Vec::<WLResult>::new();
    let copy_url: bool = get_extension_setting(EXTENSION_ID, "copy-url").unwrap() == "true";

    if !copy_url {
        for group in settings.groups {
            if fuzzy_matches(&group.name, search_text) {
                let mut result = TextResult::new(
                    &group.name,
                    Action::new_extension(
                        ExtensionAction::new(EXTENSION_ID, "open-group")
                            .args(vec![group.id.to_string()]),
                    ),
                );

                if let Some(icon_path) = group.icon_path {
                    result.icon(&icon_path);

                    if group.tint_icon {
                        result.tint("accent");
                    }
                } else {
                    result.icon(get_icon_path("folder"));
                    result.tint("accent");
                }

                results.push(WLResult::new_text(result));
            }
        }
    }

    for bookmark in settings.bookmarks {
        if fuzzy_matches(&bookmark.name, search_text) {
            let mut result = match copy_url {
                true => TextResult::new(
                    &bookmark.name,
                    Action::new_copy(CopyAction::new(&bookmark.url)),
                ),
                false => TextResult::new(
                    &bookmark.name,
                    Action::new_open_url(OpenURLAction::new(&bookmark.url)),
                ),
            };

            if let Some(icon_path) = bookmark.icon_path {
                result.icon(&icon_path);
            } else {
                result.icon(get_icon_path("bookmark"));
                result.tint("accent");
            }

            results.push(WLResult::new_text(result));
        }
    }

    send_response(results);
}
