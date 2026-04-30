from pathlib import Path

from src.deck_management.deck_locator import resolve_deck_paths, resolve_repo_root


def test_resolve_repo_root_from_tests_directory():
    repo_root = resolve_repo_root(Path(__file__).resolve())

    assert (repo_root / "pyproject.toml").exists()
    assert (repo_root / "decks").exists()


def test_resolve_deck_paths_from_deck_id():
    deck_paths = resolve_deck_paths("example-deck")

    assert deck_paths.deck_dir.name == "example-deck"
    assert deck_paths.definition.deck_id == "example-deck"
    assert deck_paths.slides_path.name == "slides.md"
    assert deck_paths.manuscript_path.name == "manuscript.md"
    assert deck_paths.theme_path == deck_paths.repo_root / "themes" / "default.css"
