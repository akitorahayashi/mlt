## Overview

mlt stands for Marp, Layout, Template. This repository manages presentation source files by deck and exports Marp slide markdown through a local Rust CLI.

## Structure

```text
decks/<deck-id>/
  manuscript.md
  slides.md
  theme.css
  assets/
  artifacts/
src/
```

- `decks/<deck-id>/manuscript.md` stores the presentation narrative.
- `decks/<deck-id>/slides.md` stores the Marp-ready slide markdown.
- `decks/<deck-id>/theme.css` stores deck-local overrides layered on the shared theme.
- `decks/<deck-id>/assets/` stores files that belong only to that deck.
- `decks/<deck-id>/artifacts/` stores exported files for that deck.
- `src/assets/default.css` stores the shared theme entrypoint used on export.
- `src/assets/theme.css.tpl` stores the empty deck override scaffold copied by `create`.
- `src/assets/css/` stores the shared category CSS used on export.

## Prerequisites

- Rust toolchain
- [marp-cli](https://github.com/marp-team/marp-cli)
- [just](https://github.com/casey/just)

An npm installation of Marp CLI and a Homebrew installation of `just` are compatible with the current commands.

```bash
npm install -g @marp-team/marp-cli
brew install just
```

## Setup

```bash
just setup
```

## Deck Workflow

The repository keeps manuscript management and slide conversion separate.

- `just run cr <deck-id>` scaffolds a new deck directory with `theme.css`, `manuscript.md`, `slides.md`, `assets/`, and `artifacts/`.
- `just run r <deck-path>` exports slides from an explicit deck directory path. The path must be absolute or relative to the workspace root (e.g., `decks/my-deck` or `tests/fixtures/test-fixture-deck`).
- `just run r <deck-path> [--pdf] [--html] [--pptx]` exports to specified formats. If no format flags are provided, all formats (HTML, PDF, PPTX) are exported by default.
- Exports are written to `<deck-path>/artifacts/`.

## Testing

The test suite validates fixture decks and export behavior.

- Unit tests in `src/` verify CLI format selection and CSS materialization logic.
- Integration tests in `tests/cli/` exercise the full export pipeline with a complete fixture deck.
- Fixture deck `tests/fixtures/test-fixture-deck/` contains a manuscript, Marp slides, custom theme CSS with specific styling, and a 1600×900 test image.
- Tests verify that exported HTML, PDF, and PPTX files are created and contain expected content (slide text, styling, image references).
- Exports are written to `tests/fixtures/test-fixture-deck/artifacts/` for inspection.
- Run `cargo test` to execute the full test suite.

## Commands

```bash
just run cr my-new-deck
just run r decks/my-new-deck
just run r decks/my-new-deck --pdf
just run r decks/my-new-deck --html --pdf
just run r tests/fixtures/test-fixture-deck
```

## Development

- `just help` prints the available recipes.
- `just run ...` invokes the local CLI through `cargo run --`.
- `just test` runs the Rust test suite.
- `just fix` runs `cargo fmt` and `cargo clippy --fix`.
- `just check` runs `cargo fmt --check` and `cargo clippy`.
