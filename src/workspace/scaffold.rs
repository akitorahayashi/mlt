use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};

use super::manifest::validate_id;
use super::resolve;
use super::Workspace;

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

    let title = humanize(id);
    fs::write(
        deck_dir.join("deck.yml"),
        format!(
            "deck_id: {id}\n\
title: {title}\n\
theme: default\n\
slides: slides.md\n\
manuscript: manuscript.md\n\
output_basename: slides\n"
        ),
    )?;

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
