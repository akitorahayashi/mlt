use std::path::{Path, PathBuf};

use crate::error::AppResult;
use crate::marp::{self, Target};
use crate::workspace;

pub fn run(root: &Path, reference: &str, target: Target) -> AppResult<Vec<PathBuf>> {
    let workspace = workspace::resolve(root, reference)?;
    marp::export_many(
        &workspace.slides_path,
        &workspace.output_dir,
        &workspace.manifest.output_basename,
        Some(&workspace.theme_path),
        target,
    )
}
