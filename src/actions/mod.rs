use std::process::exit;

use whiskers_launcher_rs::{
    api::extensions::{get_extension_dialog_response, Context},
    others::send_notification,
};

use crate::{
    bookmarks::{get_bookmarks, write_bookmarks, Bookmark},
    groups::{get_groups, write_groups, Group},
};

pub fn handle_actions(context: Context) {
    let action = context.to_owned().extension_action.unwrap();

    if action == "create_bookmark" {
        create_bookmark();
    }

    if action == "delete_bookmark" {
        delete_bookmark(context.to_owned());
    }

    if action == "edit_bookmark" {
        edit_bookmark();
    }

    if action == "create_group" {
        create_group()
    }

    if action == "delete_group" {
        delete_group(context.to_owned());
    }

    if action == "edit_group" {
        edit_group();
    }

    if action == "open_group" {
        open_group(context.to_owned())
    }
}

fn create_bookmark() {
    let response = get_extension_dialog_response().unwrap();
    let name = response.to_owned().get_result_value("name").unwrap();
    let url = response.to_owned().get_result_value("url").unwrap();

    if name.is_empty() || url.is_empty() {
        send_notification("Bookmarks", "Fields must not be empty");
        exit(0);
    }

    let mut bookmarks = get_bookmarks();

    if let Some(bigger_bookmark) = bookmarks.iter().max_by_key(|b| b.id) {
        bookmarks.push(Bookmark::new(bigger_bookmark.id + 1, name, url));
    } else {
        bookmarks.push(Bookmark::new(0, name, url));
    }

    write_bookmarks(bookmarks);
    send_notification("Bookmarks", "Bookmark created successfully");

    exit(1);
}

fn delete_bookmark(context: Context) {
    let args = context.custom_args;
    let id = args[0].to_owned();
    let bookmarks: Vec<Bookmark> = get_bookmarks()
        .iter()
        .map(|b| b.to_owned())
        .filter(|b| b.id.to_string() != id)
        .collect();

    let mut groups = Vec::<Group>::new();

    for group in get_groups() {
        if group.bookmarks.iter().any(|b| b.to_string() == id) {
            let mut new_group = group;
            new_group.bookmarks = new_group
                .bookmarks
                .to_owned()
                .iter()
                .map(|b| b.to_owned())
                .filter(|b| b.to_string() != id)
                .collect();

            groups.push(new_group);
        } else {
            groups.push(group);
        }
    }

    write_bookmarks(bookmarks);
    write_groups(groups);
    exit(0);
}

fn edit_bookmark() {
    let response = get_extension_dialog_response().unwrap();
    let name = response.to_owned().get_result_value("name").unwrap();
    let url = response.to_owned().get_result_value("url").unwrap();
    let id = response.to_owned().args.unwrap()[0].to_owned();

    if name.trim().is_empty() || url.trim().is_empty() {
        send_notification("Bookmarks", "Can't have empty fields");
        exit(0);
    }

    let mut bookmarks = Vec::<Bookmark>::new();

    for bookmark in get_bookmarks() {
        if bookmark.id.to_string() == id {
            bookmarks.push(Bookmark::new(bookmark.id, name.to_owned(), url.to_owned()))
        } else {
            bookmarks.push(bookmark);
        }
    }

    write_bookmarks(bookmarks);
    send_notification("Bookmarks", "Bookmark edited successfully");
    exit(1);
}

fn create_group() {
    let response = get_extension_dialog_response().unwrap();
    let results = response.results;
    let mut name = "".to_string();
    let mut bookmarks_ids = Vec::<usize>::new();

    for result in results {
        if result.id == "name" {
            if result.value.is_empty() {
                send_notification("Bookmarks", "Can't have empty group name");
                exit(0);
            }

            name = result.value.to_owned();
        }

        if result.value == "true" {
            bookmarks_ids.push(result.id.parse().unwrap())
        }
    }

    let mut groups = get_groups();

    if let Some(bigger_group) = groups.iter().max_by_key(|g| g.id) {
        groups.push(Group::new(bigger_group.id + 1, name, bookmarks_ids));
    } else {
        groups.push(Group::new(0, name, bookmarks_ids));
    }

    write_groups(groups);

    send_notification("Bookmarks", "Group added successfully");
    exit(1);
}

fn delete_group(context: Context) {
    let args = context.custom_args;
    let id = args[0].to_owned();
    let groups: Vec<Group> = get_groups()
        .iter()
        .map(|g| g.to_owned())
        .filter(|g| g.id.to_string() != id)
        .collect();

    write_groups(groups);
}

fn edit_group() {
    let response = get_extension_dialog_response().unwrap();
    let results = response.results;
    let group_id = response.args.unwrap()[0].to_owned();

    let mut new_name = "".to_string();
    let mut bookmarks = Vec::<usize>::new();

    for result in results {
        if result.to_owned().id == "name" {
            if result.value.is_empty() {
                send_notification("Bookmarks", "Can't have empty group name");
                exit(0);
            }

            new_name = result.to_owned().value;
        }

        if result.to_owned().value == "true" {
            bookmarks.push(result.id.parse().unwrap());
        }
    }

    let mut groups = Vec::<Group>::new();

    for group in get_groups() {
        if group.id.to_string() == group_id {
            groups.push(Group::new(
                group.id,
                new_name.to_owned(),
                bookmarks.to_owned(),
            ));
        } else {
            groups.push(group);
        }
    }

    write_groups(groups);
}

fn open_group(context: Context) {
    let args = context.custom_args;
    let group_id = args[0].to_owned();
    let groups = get_groups();

    for group in groups {
        if group.id.to_string() == group_id {
            let bookmarks = group.get_bookmarks();

            for bookmark in bookmarks {
                open::that(&bookmark.url).unwrap();
            }

            exit(1);
        }
    }
}
