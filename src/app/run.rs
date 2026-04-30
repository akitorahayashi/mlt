use std::path::{Path, PathBuf};

use crate::error::AppResult;
use crate::marp::{self, Format};
use crate::workspace;

pub fn run(root: &Path, id: &str, formats: &[Format]) -> AppResult<Vec<PathBuf>> {
    let workspace = workspace::resolve(root, id)?;

    marp::export_many(
        &workspace.slides_path,
        &workspace.artifacts_dir,
        "slides",
        Some(&workspace.theme_path),
        formats,
    )
}
