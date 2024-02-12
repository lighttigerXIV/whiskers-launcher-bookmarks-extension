use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
};

use crate::get_config_dir;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bookmark {
    pub id: usize,
    pub name: String,
    pub url: String,
}

impl Bookmark {
    pub fn new(id: usize, name: impl Into<String>, url: impl Into<String>) -> Self {
        return Self {
            id,
            name: name.into(),
            url: url.into(),
        };
    }
}

// ===================================
// Functions
// ===================================
pub fn get_bookmarks_path() -> PathBuf {
    let mut path = get_config_dir();
    path.push("bookmarks.json");

    return path;
}

pub fn get_bookmarks() -> Vec<Bookmark> {
    let path = get_bookmarks_path();

    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(bookmarks) = serde_json::from_str::<Vec<Bookmark>>(&content) {
            return bookmarks;
        }
    }

    return vec![];
}

pub fn write_bookmarks(bookmarks: Vec<Bookmark>){
    let config_dir = get_config_dir();

    if !config_dir.exists(){
        fs::create_dir_all(&config_dir).unwrap();
    }

    let bookmarks_json = serde_json::to_string_pretty(&bookmarks).unwrap();
    fs::write(&get_bookmarks_path(), &bookmarks_json).unwrap();
}