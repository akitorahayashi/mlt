from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path

LOWER_KEBAB_CASE_PATTERN = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")


@dataclass(frozen=True)
class DeckDefinition:
    deck_id: str
    title: str
    theme: str
    slides: str
    manuscript: str
    output_basename: str

    @classmethod
    def load(cls, definition_path: Path) -> "DeckDefinition":
        fields = _parse_flat_yaml_mapping(definition_path)
        allowed_fields = {
            "deck_id",
            "title",
            "theme",
            "slides",
            "manuscript",
            "output_basename",
        }
        required_fields = allowed_fields

        unknown_fields = sorted(set(fields) - allowed_fields)
        if unknown_fields:
            joined = ", ".join(unknown_fields)
            raise ValueError(f"Unknown deck.yml field(s): {joined}")

        missing_fields = sorted(required_fields - set(fields))
        if missing_fields:
            joined = ", ".join(missing_fields)
            raise ValueError(f"Missing required deck.yml field(s): {joined}")

        definition = cls(
            deck_id=fields["deck_id"],
            title=fields["title"],
            theme=fields["theme"],
            slides=fields["slides"],
            manuscript=fields["manuscript"],
            output_basename=fields["output_basename"],
        )
        definition.validate()
        return definition

    def validate(self) -> None:
        if not LOWER_KEBAB_CASE_PATTERN.fullmatch(self.deck_id):
            raise ValueError(
                "deck_id must be lower-kebab-case, for example: kyoto-go-63"
            )
        for field_name in (
            "title",
            "theme",
            "slides",
            "manuscript",
            "output_basename",
        ):
            if not getattr(self, field_name).strip():
                raise ValueError(f"{field_name} must not be empty")


def _parse_flat_yaml_mapping(definition_path: Path) -> dict[str, str]:
    if not definition_path.exists():
        raise FileNotFoundError(f"deck.yml was not found: {definition_path}")

    fields: dict[str, str] = {}
    for line_number, raw_line in enumerate(
        definition_path.read_text(encoding="utf-8").splitlines(), start=1
    ):
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        if ":" not in line:
            raise ValueError(
                f"deck.yml line {line_number} must be a flat 'key: value' entry"
            )

        key, raw_value = line.split(":", 1)
        normalized_key = key.strip()
        normalized_value = _strip_matching_quotes(raw_value.strip())
        if not normalized_key:
            raise ValueError(f"deck.yml line {line_number} has an empty key")
        if not normalized_value:
            raise ValueError(
                f"deck.yml line {line_number} has an empty value for '{normalized_key}'"
            )
        fields[normalized_key] = normalized_value
    return fields


def _strip_matching_quotes(value: str) -> str:
    if len(value) >= 2 and value[0] == value[-1] and value[0] in {"'", '"'}:
        return value[1:-1]
    return value
