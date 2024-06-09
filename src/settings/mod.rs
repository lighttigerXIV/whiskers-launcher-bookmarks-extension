use serde::{Deserialize, Serialize};

use self::functions::get_settings;

pub mod functions;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    #[serde(default = "default_groups")]
    pub groups: Vec<Group>,
    #[serde(default = "default_bookmarks")]
    pub bookmarks: Vec<Bookmark>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: usize,
    pub icon_path: Option<String>,
    pub tint_icon: bool,
    pub name: String,
    pub bookmarks_ids: Vec<usize>,
}

impl Group {
    pub fn new(name: impl Into<String>, bookmarks_ids: Vec<usize>) -> Self {
        let groups = get_settings().groups;
        let last_group = groups.iter().max_by_key(|group| group.id);

        let new_id = match last_group {
            Some(group) => group.id + 1,
            None => 0,
        };

        Self {
            id: new_id,
            icon_path: None,
            tint_icon: false,
            name: name.into(),
            bookmarks_ids,
        }
    }

    pub fn icon_path(&mut self, icon_path: impl Into<String>) -> Self {
        self.icon_path = Some(icon_path.into());
        self.to_owned()
    }

    pub fn tint_icon(&mut self, tint_icon: bool) -> Self {
        self.tint_icon = tint_icon;
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bookmark {
    pub id: usize,
    pub icon_path: Option<String>,
    pub name: String,
    pub url: String,
}

impl Bookmark {
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        let bookmarks = get_settings().bookmarks;
        let last_bookmark = bookmarks.iter().max_by_key(|bookmark| bookmark.id);
        let new_id = match last_bookmark {
            Some(bookmark) => bookmark.id + 1,
            None => 0,
        };

        Self {
            id: new_id,
            icon_path: None,
            name: name.into(),
            url: url.into(),
        }
    }

    pub fn icon_path(&mut self, icon_path: impl Into<String>) -> Self {
        self.icon_path = Some(icon_path.into());
        self.to_owned()
    }
}

fn default_groups() -> Vec<Group> {
    Vec::new()
}

fn default_bookmarks() -> Vec<Bookmark> {
    Vec::new()
}
