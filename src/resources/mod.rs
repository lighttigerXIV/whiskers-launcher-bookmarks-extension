use std::path::PathBuf;
use simple_kl_rs::paths::get_extension_directory;

pub const EXTENSION_ID: &str = "com-lighttigerxiv-bookmarks";

pub fn get_icons_dir()-> Option<PathBuf>{
    let mut path = get_extension_directory(EXTENSION_ID)?;
    path.push("src");
    path.push("resources");
    path.push("icons");

    return Some(path)
}

pub fn get_bookmark_icon()-> Option<PathBuf>{
    let mut path = get_icons_dir()?;
    path.push("bookmark.svg");
    
    return Some(path)
}

pub fn get_folder_icon()-> Option<PathBuf>{
    let mut path = get_icons_dir()?;
    path.push("folder.svg");
    
    return Some(path)
}

pub fn get_pencil_icon()-> Option<PathBuf>{
    let mut path = get_icons_dir()?;
    path.push("pencil.svg");
    
    return Some(path)
}

pub fn get_plus_icon()-> Option<PathBuf>{
    let mut path = get_icons_dir()?;
    path.push("plus.svg");
    
    return Some(path)
}

pub fn get_trash_icon()-> Option<PathBuf>{
    let mut path = get_icons_dir()?;
    path.push("trash.svg");
    
    return Some(path)
}