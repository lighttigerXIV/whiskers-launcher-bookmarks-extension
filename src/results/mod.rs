use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use whiskers_launcher_rs::{
    actions,
    api::extensions::{get_extension_setting, send_extension_results, Context},
    dialog::{self, DialogField},
    results::{self, WhiskersResult},
};

use crate::{bookmarks::get_bookmarks, groups::get_groups, resources::get_icon, EXTENSION_ID};

pub fn handle_results(context: Context) {
    let typed_text = context.search_text.unwrap();

    if typed_text.is_empty() {
        show_main_results();
    }

    if typed_text.contains(" ") {
        let mut keyword = "".to_string();
        let mut search_text = "".to_string();
        let mut has_keyword = false;

        for typed_char in typed_text.chars() {
            if typed_char == ' ' && !has_keyword {
                has_keyword = true;
            } else if !has_keyword {
                keyword += &typed_char.to_string();
            } else {
                search_text += &typed_char.to_string();
            }
        }

        search_text = search_text.trim().to_string();

        if keyword == "d" || keyword == "delete" {
            show_delete_results(&search_text)
        } else if keyword == "e" || keyword == "edit" {
            show_edit_results(&search_text);
        } else {
            show_search_results(&search_text);
        }
    }

    show_search_results(&typed_text);
}

fn show_main_results() {
    let mut results = Vec::<WhiskersResult>::new();
    let mut add_bookmark_fields = Vec::<DialogField>::new();
    let mut creat_group_fields = Vec::<DialogField>::new();
    let copy_url = as_bool(get_extension_setting(EXTENSION_ID, "copy_url").unwrap());

    add_bookmark_fields.push(DialogField::Input(
        dialog::Input::new("name", "Name", "")
            .description("The bookmark name")
            .placeholder("Name"),
    ));

    add_bookmark_fields.push(DialogField::Input(
        dialog::Input::new("url", "Url", "")
            .description("The bookmark url")
            .placeholder("Url"),
    ));

    if !copy_url {
        creat_group_fields.push(DialogField::Input(
            dialog::Input::new("name", "Name", "")
                .description("The group name")
                .placeholder("Name"),
        ));

        for bookmark in get_bookmarks() {
            creat_group_fields.push(DialogField::Toggle(
                dialog::Toggle::new(&bookmark.id.to_string(), &bookmark.name, false)
                    .description("Toggle if you want to add the bookmark to the group"),
            ))
        }
    }

    results.push(WhiskersResult::Text(
        results::Text::new(
            "Create bookmark",
            actions::Action::Dialog(
                actions::Dialog::new(
                    EXTENSION_ID,
                    "Create Bookmark",
                    "create_bookmark",
                    add_bookmark_fields,
                )
                .primary_button_text("Create Bookmark"),
            ),
        )
        .icon(get_icon("plus.svg"))
        .tint_icon(true),
    ));

    if !copy_url {
        results.push(WhiskersResult::Text(
            results::Text::new(
                "Create group",
                actions::Action::Dialog(
                    actions::Dialog::new(
                        EXTENSION_ID,
                        "Create Group",
                        "create_group",
                        creat_group_fields,
                    )
                    .primary_button_text("Create Group"),
                ),
            )
            .icon(get_icon("plus.svg"))
            .tint_icon(true),
        ));
    }

    send_extension_results(results);
}

fn show_search_results(search_text: impl Into<String>) {
    let search_text = search_text.into();
    let mut results = Vec::<WhiskersResult>::new();
    let bookmarks = get_bookmarks();
    let groups = get_groups();
    let matcher = SkimMatcherV2::default();
    let copy_url = as_bool(get_extension_setting(EXTENSION_ID, "copy_url").unwrap());

    if !copy_url {
        for group in groups {
            if matcher.fuzzy_match(&group.name, &search_text).is_some() {
                results.push(WhiskersResult::Text(
                    results::Text::new(
                        format!("Open {}", &group.name),
                        actions::Action::Extension(
                            actions::Extension::new(EXTENSION_ID, "open_group")
                                .args(vec![group.id.to_string()]),
                        ),
                    )
                    .icon(get_icon("dir.svg"))
                    .tint_icon(true),
                ));
            }
        }
    }

    for bookmark in bookmarks {
        if matcher.fuzzy_match(&bookmark.name, &search_text).is_some() {
            if copy_url {
                results.push(WhiskersResult::Text(
                    results::Text::new(
                        format!("Copy {}", &bookmark.url),
                        actions::Action::CopyToClipboard(actions::CopyToClipboard::new(
                            &bookmark.url,
                        )),
                    )
                    .icon(get_icon("bookmark.svg"))
                    .tint_icon(true),
                ));
            } else {
                results.push(WhiskersResult::Text(
                    results::Text::new(
                        format!("Open {}", &bookmark.name),
                        actions::Action::OpenUrl(actions::OpenUrl::new(&bookmark.url)),
                    )
                    .icon(get_icon("bookmark.svg"))
                    .tint_icon(true),
                ));
            }
        }
    }

    send_extension_results(results);
}

fn show_delete_results(search_text: impl Into<String>) {
    let search_text = search_text.into();
    let mut results = Vec::<WhiskersResult>::new();
    let bookmarks = get_bookmarks();
    let groups = get_groups();
    let matcher = SkimMatcherV2::default();

    for group in groups {
        if matcher.fuzzy_match(&group.name, &search_text).is_some() {
            results.push(WhiskersResult::Text(
                results::Text::new(
                    format!("Delete {} Group", &group.name),
                    actions::Action::Extension(
                        actions::Extension::new(EXTENSION_ID, "delete_group")
                            .args(vec![group.id.to_string()]),
                    ),
                )
                .icon(get_icon("trash.svg"))
                .tint_icon(true),
            ))
        }
    }

    for bookmark in bookmarks {
        if matcher.fuzzy_match(&bookmark.name, &search_text).is_some() {
            results.push(WhiskersResult::Text(
                results::Text::new(
                    format!("Delete {}", &bookmark.name),
                    actions::Action::Extension(
                        actions::Extension::new(EXTENSION_ID, "delete_bookmark")
                            .args(vec![bookmark.id.to_string()]),
                    ),
                )
                .icon(get_icon("trash.svg"))
                .tint_icon(true),
            ))
        }
    }

    send_extension_results(results);
}

fn show_edit_results(search_text: impl Into<String>) {
    let search_text = search_text.into();
    let mut results = Vec::<WhiskersResult>::new();
    let bookmarks = get_bookmarks();
    let groups = get_groups();
    let matcher = SkimMatcherV2::default();

    if search_text.is_empty() {
        send_extension_results(vec![]);
    }

    for group in groups {
        if matcher.fuzzy_match(&group.name, &search_text).is_some() {
            let mut fields = Vec::<DialogField>::new();

            fields.push(DialogField::Input(
                dialog::Input::new("name", "Name", &group.name).description("The group name"),
            ));

            for bookmark in bookmarks.to_owned() {
                fields.push(DialogField::Toggle(
                    dialog::Toggle::new(
                        bookmark.id.to_string(),
                        bookmark.name,
                        group
                            .bookmarks
                            .iter()
                            .map(|b| b.to_owned())
                            .any(|b| b == bookmark.id),
                    )
                    .description("Toggle if you want to add the bookmark to the group"),
                ));
            }

            results.push(WhiskersResult::Text(
                results::Text::new(
                    format!("Edit {} Group", &group.name),
                    actions::Action::Dialog(
                        actions::Dialog::new(
                            EXTENSION_ID,
                            format!("Edit {} Group", &group.name),
                            "edit_group",
                            fields,
                        )
                        .args(vec![group.id.to_string()])
                        .primary_button_text("Save"),
                    ),
                )
                .icon(get_icon("pencil.svg"))
                .tint_icon(true),
            ))
        }
    }

    for bookmark in bookmarks {
        if matcher.fuzzy_match(&bookmark.name, &search_text).is_some() {
            let mut fields = Vec::<DialogField>::new();

            fields.push(DialogField::Input(
                dialog::Input::new("name", "Name", &bookmark.name).description("The bookmark name"),
            ));

            fields.push(DialogField::Input(
                dialog::Input::new("url", "Url", &bookmark.url).description("The bookmark url"),
            ));

            results.push(WhiskersResult::Text(
                results::Text::new(
                    format!("Edit {}", &bookmark.name),
                    actions::Action::Dialog(
                        actions::Dialog::new(
                            EXTENSION_ID,
                            format!("Edit {}", &bookmark.name),
                            "edit_bookmark",
                            fields,
                        )
                        .args(vec![bookmark.id.to_string()])
                        .primary_button_text("Save"),
                    ),
                )
                .icon(get_icon("pencil.svg"))
                .tint_icon(true),
            ))
        }
    }

    send_extension_results(results);
}

fn as_bool(value: String) -> bool {
    match value.as_str() {
        "true" => true,
        _ => false,
    }
}
