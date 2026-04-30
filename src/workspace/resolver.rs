use std::path::{Path, PathBuf};

use crate::error::{AppError, AppResult};

use super::manifest::Manifest;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub root: PathBuf,
    pub deck_dir: PathBuf,
    pub manifest_path: PathBuf,
    pub manuscript_path: PathBuf,
    pub slides_path: PathBuf,
    pub output_dir: PathBuf,
    pub theme_path: PathBuf,
    pub manifest: Manifest,
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
    let deck_dir = resolve_directory(root, reference)?;
    let manifest_path = deck_dir.join("deck.yml");
    let manifest = Manifest::load(&manifest_path)?;

    if deck_dir
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .as_deref()
        != Some(&manifest.deck_id)
    {
        return Err(AppError::InvalidManifest(format!(
            "deck directory name and deck_id must match: {} != {}",
            deck_dir.display(),
            manifest.deck_id
        )));
    }

    let manuscript_path = deck_dir.join(&manifest.manuscript);
    let slides_path = deck_dir.join(&manifest.slides);
    let theme_path = root.join("themes").join(format!("{}.css", manifest.theme));
    let output_dir = deck_dir.join("artifacts");

    ensure_exists("Manuscript", &manuscript_path)?;
    ensure_exists("Slides", &slides_path)?;
    ensure_exists("Theme", &theme_path)?;

    Ok(Workspace {
        root: root.to_path_buf(),
        deck_dir,
        manifest_path,
        manuscript_path,
        slides_path,
        output_dir,
        theme_path,
        manifest,
    })
}

fn resolve_directory(root: &Path, reference: &str) -> AppResult<PathBuf> {
    let candidate = PathBuf::from(reference);
    if candidate.exists() {
        let resolved = candidate.canonicalize()?;
        if !resolved.is_dir() {
            return Err(AppError::DeckReferenceIsNotDirectory(resolved));
        }
        return Ok(resolved);
    }

    let deck_dir = root.join("decks").join(reference);
    if !deck_dir.exists() {
        return Err(AppError::DeckNotFound(reference.to_string()));
    }
    Ok(deck_dir)
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
