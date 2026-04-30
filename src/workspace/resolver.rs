use std::path::{Path, PathBuf};

use super::scaffold::validate_id;
use crate::error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct Workspace {
    pub root: PathBuf,
    pub deck_id: String,
    pub deck_dir: PathBuf,
    pub manuscript_path: PathBuf,
    pub slides_path: PathBuf,
    pub artifacts_dir: PathBuf,
    pub theme_path: PathBuf,
}

pub fn locate_root(start: &Path) -> AppResult<PathBuf> {
    for candidate in start.ancestors() {
        if candidate.join("Cargo.toml").exists() && candidate.join("decks").is_dir() {
            return Ok(candidate.to_path_buf());
        }
    }
    Err(AppError::WorkspaceNotFound(start.to_path_buf()))
}

pub fn resolve(root: &Path, reference: &str) -> AppResult<Workspace> {
    validate_id(reference)?;
    let deck_dir = root.join("decks").join(reference);
    if !deck_dir.exists() {
        return Err(AppError::DeckNotFound(reference.to_string()));
    }

    let manuscript_path = deck_dir.join("manuscript.md");
    let slides_path = deck_dir.join("slides.md");
    let theme_path = deck_dir.join("default.css");
    let artifacts_dir = deck_dir.join("artifacts");

    ensure_exists("Manuscript", &manuscript_path)?;
    ensure_exists("Slides", &slides_path)?;
    ensure_exists("Theme", &theme_path)?;

    Ok(Workspace {
        root: root.to_path_buf(),
        deck_id: reference.to_string(),
        deck_dir,
        manuscript_path,
        slides_path,
        artifacts_dir,
        theme_path,
    })
}

fn ensure_exists(kind: &'static str, path: &Path) -> AppResult<()> {
    if path.exists() {
        return Ok(());
    }
    Err(AppError::MissingPath {
        kind,
        path: path.to_path_buf(),
    })
}
