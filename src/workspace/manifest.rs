use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub deck_id: String,
    pub title: String,
    pub theme: String,
    pub slides: String,
    pub manuscript: String,
    pub output_basename: String,
}

impl Manifest {
    pub fn load(path: &Path) -> AppResult<Self> {
        let content = fs::read_to_string(path)?;
        let manifest =
            serde_yaml::from_str::<Self>(&content).map_err(|source| AppError::ManifestParse {
                path: path.to_path_buf(),
                source,
            })?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> AppResult<()> {
        validate_id(&self.deck_id)?;
        validate_non_empty("title", &self.title)?;
        validate_non_empty("theme", &self.theme)?;
        validate_non_empty("slides", &self.slides)?;
        validate_non_empty("manuscript", &self.manuscript)?;
        validate_non_empty("output_basename", &self.output_basename)?;
        Ok(())
    }
}

pub fn validate_id(value: &str) -> AppResult<()> {
    if is_lower_kebab_case(value) {
        return Ok(());
    }
    Err(AppError::InvalidDeckId(value.to_string()))
}

fn validate_non_empty(field: &str, value: &str) -> AppResult<()> {
    if value.trim().is_empty() {
        return Err(AppError::InvalidManifest(format!(
            "{field} must not be empty"
        )));
    }
    Ok(())
}

fn is_lower_kebab_case(value: &str) -> bool {
    let mut parts = value.split('-');
    let Some(first) = parts.next() else {
        return false;
    };
    if first.is_empty()
        || !first
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    {
        return false;
    }
    parts.all(|part| {
        !part.is_empty()
            && part
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    })
}
