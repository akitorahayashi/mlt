use std::fs;
use std::path::Path;

use crate::error::AppResult;

use super::resolve;

pub fn list(root: &Path) -> AppResult<Vec<String>> {
    let decks_dir = root.join("decks");
    let mut ids = Vec::new();

    for item in fs::read_dir(&decks_dir)? {
        let item = item?;
        if item.file_type()?.is_dir() {
            ids.push(item.file_name().to_string_lossy().to_string());
        }
    }
    ids.sort();

    let mut valid_ids = Vec::new();
    for id in ids {
        if resolve(root, &id).is_ok() {
            valid_ids.push(id);
        }
    }

    Ok(valid_ids)
}
