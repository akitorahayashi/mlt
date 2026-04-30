use std::path::{Path, PathBuf};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct DeckLayout {
    pub deck_id: String,
    pub deck_dir: PathBuf,
    pub manuscript_path: PathBuf,
    pub slides_path: PathBuf,
    pub artifacts_dir: PathBuf,
    pub theme_path: PathBuf,
}

pub fn resolve_dir(deck_dir: &Path) -> AppResult<DeckLayout> {
    if !deck_dir.exists() || !deck_dir.is_dir() {
        return Err(AppError::DeckNotFound(deck_dir.display().to_string()));
    }

    let manuscript_path = deck_dir.join("manuscript.md");
    let slides_path = deck_dir.join("slides.md");
    let theme_path = deck_dir.join("theme.css");
    let artifacts_dir = deck_dir.join("artifacts");

    ensure_exists("Manuscript", &manuscript_path, PathKind::File)?;
    ensure_exists("Slides", &slides_path, PathKind::File)?;
    ensure_exists("Theme", &theme_path, PathKind::File)?;
    ensure_exists("Artifacts", &artifacts_dir, PathKind::Directory)?;

    let deck_id = deck_dir
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .filter(|id| !id.is_empty())
        .unwrap_or_else(|| "slides".to_string());

    Ok(DeckLayout {
        deck_id,
        deck_dir: deck_dir.to_path_buf(),
        manuscript_path,
        slides_path,
        artifacts_dir,
        theme_path,
    })
}

fn ensure_exists(kind: &'static str, path: &Path, expected_kind: PathKind) -> AppResult<()> {
    let is_valid = match expected_kind {
        PathKind::File => path.is_file(),
        PathKind::Directory => path.is_dir(),
    };
    if is_valid {
        return Ok(());
    }
    Err(AppError::MissingPath {
        kind,
        path: path.to_path_buf(),
    })
}

enum PathKind {
    File,
    Directory,
}
