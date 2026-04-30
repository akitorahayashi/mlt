use std::path::{Path, PathBuf};

use crate::error::AppResult;
use crate::marp::{self, Target};

pub fn run(
    _root: &Path,
    slides_path: &Path,
    output_dir: &Path,
    theme: Option<&Path>,
    basename: Option<&str>,
    target: Target,
) -> AppResult<Vec<PathBuf>> {
    let derived_basename = basename
        .map(ToOwned::to_owned)
        .or_else(|| {
            slides_path
                .file_stem()
                .map(|stem| stem.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "slides".to_string());

    marp::export_many(slides_path, output_dir, &derived_basename, theme, target)
}
