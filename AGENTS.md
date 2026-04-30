# marp-pj Integration Notes for AI Agents

## Project Role
- `marp-pj` manages local deck directories and exports completed Marp markdown through a Rust CLI.
- AI agents own slide composition from `manuscript.md` into `slides.md`.

## Key Files
- `decks/<deck-id>/manuscript.md`
  - Deck-specific manuscript and speaking material.
- `decks/<deck-id>/slides.md`
  - Deck-specific Marp-ready slide source.
- `decks/<deck-id>/default.css`
  - Deck-specific Marp theme file used on export.
- `decks/<deck-id>/artifacts/`
  - Deck-specific export outputs.
- `src/assets/default.css`
  - Scaffold template copied into new decks. Slide front matter uses `marp-pj-default`.

## Expected Workflow
1. Read `decks/<deck-id>/manuscript.md`.
2. Write a coherent deck to `decks/<deck-id>/slides.md`.
3. Keep each slide centered on one idea and avoid paragraph-heavy output.

## Export Interface
- Human-facing commands go through `just run ...`.
- Common commands:
  - `just run ls`
  - `just run cr <deck-id>`
  - `just run r <deck-id>`
  - `just run r <deck-id> [--pdf] [--html] [--pptx]`

## Structural Changes
- New deck scaffolds belong under `decks/<deck-id>/`.
