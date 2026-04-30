# mlt Integration Notes for AI Agents

## Project Role
- `mlt` stands for Marp, Layout, Template. It manages local deck directories and exports completed Marp markdown through a Rust CLI.
- AI agents own slide composition from `manuscript.md` into `slides.md`.

## Key Files
- `decks/<deck-id>/manuscript.md`
  - Deck-specific manuscript and speaking material.
- `decks/<deck-id>/slides.md`
  - Deck-specific Marp-ready slide source.
- `decks/<deck-id>/theme.css`
  - Deck-specific overrides layered on the shared theme during export.
- `decks/<deck-id>/artifacts/`
  - Deck-specific export outputs.
- `src/assets/default.css`
  - Shared theme entrypoint used on export. Slide front matter uses `mlt-default`.
- `src/assets/theme.css.tpl`
  - Scaffold override template copied into new decks.
- `src/assets/css/`
  - Shared category CSS used on export.

## Expected Workflow
1. Read `decks/<deck-id>/manuscript.md`.
2. Write a coherent deck to `decks/<deck-id>/slides.md`.
3. Keep each slide centered on one idea and avoid paragraph-heavy output.

## Export Interface
- Human-facing commands go through `just run ...`.
- `just run cr <deck-id>` scaffolds a new deck under `decks/<deck-id>/`.
- `just run r <deck-path>` exports a deck from an explicit directory path (e.g., `decks/my-deck`, `tests/fixtures/test-fixture-deck`).
- `just run r <deck-path> [--pdf] [--html] [--pptx]` exports to specified formats; all formats export by default.
- Exported files are written to `<deck-path>/artifacts/`.
- Path resolution validates the deck directory contains `manuscript.md`, `slides.md`, `theme.css`, and `artifacts/`; missing files cause explicit failure.
