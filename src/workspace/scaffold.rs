use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};

use super::resolve;
use super::Workspace;

const THEME_CSS: &str = include_str!("../assets/theme.css.tpl");
const MANUSCRIPT_TEMPLATE: &str = include_str!("../assets/manuscript.md.tpl");
const SLIDES_TEMPLATE: &str = include_str!("../assets/slides.md.tpl");

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
    fs::write(deck_dir.join("theme.css"), THEME_CSS)?;
    let title = humanize(id);

    let manuscript = MANUSCRIPT_TEMPLATE.replace("__TITLE__", &title);
    fs::write(deck_dir.join("manuscript.md"), manuscript)?;

    let slides = SLIDES_TEMPLATE
        .replace("__TITLE__", &title)
        .replace("__DECK_ID__", id);
    fs::write(deck_dir.join("slides.md"), slides)?;

    resolve(root, id)
}

pub fn validate_id(value: &str) -> AppResult<()> {
    if is_lower_kebab_case(value) {
        return Ok(());
    }
    Err(AppError::InvalidDeckId(value.to_string()))
}

fn is_lower_kebab_case(value: &str) -> bool {
    !value.is_empty()
        && value.split('-').all(|part| {
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
