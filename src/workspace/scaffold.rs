use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};

use super::resolve;
use super::Workspace;

const DEFAULT_THEME_CSS: &str = include_str!("../assets/default.css");

pub fn create(root: &Path, id: &str) -> AppResult<Workspace> {
    validate_id(id)?;

    let deck_dir = root.join("decks").join(id);
    if deck_dir.exists() {
        return Err(AppError::DeckAlreadyExists(id.to_string()));
    }

    fs::create_dir_all(deck_dir.join("assets"))?;
    fs::create_dir_all(deck_dir.join("artifacts"))?;
    fs::write(deck_dir.join("assets").join(".gitkeep"), "")?;
    fs::write(deck_dir.join("artifacts").join(".gitkeep"), "")?;
    fs::write(deck_dir.join("default.css"), DEFAULT_THEME_CSS)?;
    let title = humanize(id);

    fs::write(
        deck_dir.join("manuscript.md"),
        format!(
            "---\n\
title: {title}\n\
language: ja\n\
---\n\n\
ここに発表原稿を書く。\n"
        ),
    )?;

    fs::write(
        deck_dir.join("slides.md"),
        format!(
            "---\n\
marp: true\n\
theme: marp-pj-default\n\
paginate: true\n\
header: ''\n\
footer: ''\n\
---\n\n\
# {title}\n\n\
Deck scaffold for `{id}`.\n"
        ),
    )?;

    resolve(root, id)
}

pub fn validate_id(value: &str) -> AppResult<()> {
    if is_lower_kebab_case(value) {
        return Ok(());
    }
    Err(AppError::InvalidDeckId(value.to_string()))
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

fn humanize(id: &str) -> String {
    id.split('-')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => {
                    let mut word = first.to_ascii_uppercase().to_string();
                    word.push_str(chars.as_str());
                    word
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
