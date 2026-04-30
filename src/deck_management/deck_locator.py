from __future__ import annotations

from pathlib import Path

from .deck_definition import LOWER_KEBAB_CASE_PATTERN, DeckDefinition
from .deck_paths import DeckPaths


def resolve_repo_root(start_path: Path | None = None) -> Path:
    current = (start_path or Path.cwd()).resolve()
    for candidate in (current, *current.parents):
        if (candidate / "pyproject.toml").exists() and (candidate / "src").exists():
            return candidate
    raise FileNotFoundError("Project root with pyproject.toml and src/ was not found")


def resolve_deck_paths(
    deck_reference: str,
    repo_root: Path | None = None,
) -> DeckPaths:
    root = resolve_repo_root(repo_root or Path.cwd())
    deck_dir = _resolve_deck_directory(root, deck_reference)
    definition_path = deck_dir / "deck.yml"
    definition = DeckDefinition.load(definition_path)

    deck_paths = DeckPaths(
        repo_root=root,
        deck_dir=deck_dir,
        definition_path=definition_path,
        manuscript_path=deck_dir / definition.manuscript,
        slides_path=deck_dir / definition.slides,
        output_dir=root / "output" / definition.deck_id,
        theme_path=root / "themes" / f"{definition.theme}.css",
        definition=definition,
    )
    deck_paths.validate()
    return deck_paths


def _resolve_deck_directory(repo_root: Path, deck_reference: str) -> Path:
    candidate = Path(deck_reference)
    if candidate.exists():
        resolved_candidate = candidate.resolve()
        if not resolved_candidate.is_dir():
            raise NotADirectoryError(
                f"Deck reference must point to a deck directory: {resolved_candidate}"
            )
        return resolved_candidate

    if LOWER_KEBAB_CASE_PATTERN.fullmatch(deck_reference):
        deck_dir = (repo_root / "decks" / deck_reference).resolve()
        if not deck_dir.exists():
            raise FileNotFoundError(f"Deck directory was not found: {deck_dir}")
        return deck_dir

    raise FileNotFoundError(
        "Deck reference must be an existing deck directory or a lower-kebab-case "
        f"deck id under decks/: {deck_reference}"
    )
