use std::path::{Path, PathBuf};

use crate::error::AppResult;
use crate::marp::{self, Format, Target};
use crate::workspace;

pub fn run(root: &Path, id: &str, format: Option<Format>) -> AppResult<Vec<PathBuf>> {
    let workspace = workspace::resolve(root, id)?;
    let target = match format {
        Some(single) => Target::Single(single),
        None => Target::All,
    };

    marp::export_many(
        &workspace.slides_path,
        &workspace.artifacts_dir,
        "slides",
        Some(&workspace.theme_path),
        target,
    )
}
