use std::fs;
use std::path::Path;

use crate::error::AppResult;

use super::resolve;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub id: String,
    pub title: String,
}

pub fn list(root: &Path) -> AppResult<Vec<Entry>> {
    let decks_dir = root.join("decks");
    let mut ids = Vec::new();

    for item in fs::read_dir(&decks_dir)? {
        let item = item?;
        if item.file_type()?.is_dir() {
            ids.push(item.file_name().to_string_lossy().to_string());
        }
    }
    ids.sort();

    let mut entries = Vec::new();
    for id in ids {
        if let Ok(workspace) = resolve(root, &id) {
            entries.push(Entry {
                id: workspace.manifest.deck_id,
                title: workspace.manifest.title,
            });
        }
    }

    Ok(entries)
}
