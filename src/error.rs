use std::path::PathBuf;

use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid deck id `{0}`. Deck ids must be lower-kebab-case.")]
    InvalidDeckId(String),
    #[error("Deck not found: {0}")]
    DeckNotFound(String),
    #[error("Deck already exists: {0}")]
    DeckAlreadyExists(String),
    #[error("Workspace root with Cargo.toml and decks/ was not found from {0}")]
    WorkspaceNotFound(PathBuf),
    #[error("{kind} file was not found: {path}")]
    MissingPath { kind: &'static str, path: PathBuf },
    #[error("Marp command failed: {0}")]
    MarpCommandFailed(String),
}
