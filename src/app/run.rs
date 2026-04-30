use std::path::{Path, PathBuf};

use crate::deck_layout;
use crate::error::AppResult;
use crate::marp::{self, Format};

pub fn run(deck_dir: &Path, formats: &[Format]) -> AppResult<Vec<PathBuf>> {
    let deck_layout = deck_layout::resolve_dir(deck_dir)?;

    marp::export_many(
        &deck_layout.slides_path,
        &deck_layout.artifacts_dir,
        &deck_layout.deck_id,
        Some(&deck_layout.theme_path),
        formats,
    )
}
