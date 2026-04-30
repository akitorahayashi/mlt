from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

from .deck_definition import DeckDefinition


@dataclass(frozen=True)
class DeckPaths:
    repo_root: Path
    deck_dir: Path
    definition_path: Path
    manuscript_path: Path
    slides_path: Path
    output_dir: Path
    theme_path: Path
    definition: DeckDefinition

    def validate(self) -> None:
        if self.deck_dir.name != self.definition.deck_id:
            raise ValueError(
                "Deck directory name and deck_id must match: "
                f"{self.deck_dir.name} != {self.definition.deck_id}"
            )
        for path, label in (
            (self.manuscript_path, "Manuscript"),
            (self.slides_path, "Slides"),
            (self.theme_path, "Theme"),
        ):
            if not path.exists():
                raise FileNotFoundError(f"{label} file was not found: {path}")
