# marp-pj Integration Notes for AI Agents

## Project Role
- `marp-pj` manages local deck directories and exports completed Marp markdown through a Rust CLI.
- AI agents own slide composition from `manuscript.md` into `slides.md`.

## Key Files
- `decks/<deck-id>/manuscript.md`
  - Deck-specific manuscript and speaking material.
- `decks/<deck-id>/slides.md`
  - Deck-specific Marp-ready slide source.
- `decks/<deck-id>/deck.yml`
  - Deck metadata.
- `layouts/`
  - Shared slide patterns grouped by story phase.
- `themes/default.css`
  - Shared Marp theme. Slide front matter uses `marp-pj-default`.
- `slide-generation.md`
  - Canonical instructions for turning manuscripts into slides.

## Expected Workflow
1. Read `slide-generation.md`.
2. Read `decks/<deck-id>/manuscript.md`.
3. Reuse and adapt the patterns under `layouts/`.
4. Write a coherent deck to `decks/<deck-id>/slides.md`.
5. Keep each slide centered on one idea and avoid paragraph-heavy output.

## Export Interface
- Human-facing commands go through `just run ...`.
- Common commands:
  - `just run decks list`
  - `just run decks show <deck-id>`
  - `just run export <format> <deck-id>`
  - `just run convert <format> <slides-path> --output-dir <dir>`

## Structural Changes
- Structural slide changes belong in `layouts/`, not as one-off edits to generated deck content.
- New deck scaffolds belong under `decks/<deck-id>/`.
