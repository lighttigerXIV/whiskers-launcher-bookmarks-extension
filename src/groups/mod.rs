use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    bookmarks::{get_bookmarks, Bookmark},
    get_config_dir,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: usize,
    pub name: String,
    pub bookmarks: Vec<usize>,
}

impl Group {
    pub fn new(id: usize, name: impl Into<String>, bookmarks: Vec<usize>) -> Self {
        Self {
            id,
            name: name.into(),
            bookmarks,
        }
    }

    pub fn get_bookmarks(&self) -> Vec<Bookmark> {
        let ids = self.bookmarks.to_owned();
        let mut bookmarks = Vec::<Bookmark>::new();

        for bookmark in get_bookmarks() {
            if ids.iter().any(|b| b == &bookmark.id) {
                bookmarks.push(bookmark);
            }
        }

        bookmarks
    }
}

// ===================================
// Functions
// ===================================
pub fn get_groups_path() -> PathBuf {
    let mut path = get_config_dir();
    path.push("groups.json");

    path
}

pub fn get_groups() -> Vec<Group> {
    let path = get_groups_path();

    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(groups) = serde_json::from_str::<Vec<Group>>(&content) {
            return groups;
        }
    }

    vec![]
}

pub fn write_groups(groups: Vec<Group>) {
    let config_dir = get_config_dir();

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).unwrap();
    }

    let groups_json = serde_json::to_string_pretty(&groups).unwrap();
    fs::write(&get_groups_path(), &groups_json).unwrap();
}
