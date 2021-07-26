use crate::config::Settings;
use std::path::PathBuf;
use walkdir::{WalkDir, DirEntry};

pub fn repos(settings: &Settings) -> impl Iterator<Item = PathBuf> {
    let mut walker = WalkDir::new(&settings.base_dir);
    if let Some(min_depth) = settings.min_depth {
        walker = walker.min_depth(min_depth);
    }

    walker.into_iter().filter_entry(filter).filter_map(|e| {
        if let Ok(entry) = e {
            if entry.path().ends_with(".git") {
                return entry.path().parent().map(|p| p.to_owned());
            }
        }
        None
    })
}

fn filter(entry: &DirEntry) -> bool {
    is_directory(entry) && !is_hidden(entry)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(".") && !s.ends_with(".git"))
        .unwrap_or(false)
}

fn is_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}
