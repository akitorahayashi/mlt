use std::path::{Path, PathBuf};

use crate::error::AppResult;
use crate::marp::{self, Format};
use crate::workspace;

pub fn run(deck_dir: &Path, formats: &[Format]) -> AppResult<Vec<PathBuf>> {
    let workspace = workspace::resolve_dir(deck_dir)?;

    marp::export_many(
        &workspace.slides_path,
        &workspace.artifacts_dir,
        &workspace.deck_id,
        Some(&workspace.theme_path),
        formats,
    )
}
