#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct BookmarksFile {
    pub bookmarks: Vec<Bookmark>,
    pub groups: Vec<BookmarkGroup>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Bookmark {
    pub id: usize,
    pub name: String,
    pub(crate) url: String,
}

impl Bookmark {
    pub fn new(id: usize, name: &str, url: &str) -> Self {
        return Bookmark {
            id,
            name: name.to_owned(),
            url: url.to_owned(),
        };
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct BookmarkGroup {
    pub id: usize,
    pub name: String,
    pub bookmarks: Vec<usize>,
}

impl BookmarkGroup {
    pub fn new(id: usize, name: &str, bookmarks: Vec<usize>) -> Self {
        return BookmarkGroup {
            id,
            name: name.to_owned(),
            bookmarks,
        };
    }
}
