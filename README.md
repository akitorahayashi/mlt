## Overview

This repository manages presentation source files by deck and exports Marp slide markdown through a local Rust CLI.

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

- `just run ls` lists valid deck ids.
- `just run cr <deck-id>` scaffolds a deck directory with `theme.css`, `manuscript.md`, `slides.md`, and the `assets/` / `artifacts/` directories.
- `just run r <deck-id>` exports all supported formats into `decks/<deck-id>/artifacts/`.
- `just run r <deck-id> [--pdf] [--html] [--pptx]` exports selected formats.

`macos-defaults-plist` is the starter deck under `decks/macos-defaults-plist/`.

## Commands

```bash
just run ls
just run cr kyoto-go-64
just run r macos-defaults-plist
just run r macos-defaults-plist --pdf
just run r macos-defaults-plist --html --pdf
```

## Development

- `just help` prints the available recipes.
- `just run ...` invokes the local CLI through `cargo run --`.
- `just test` runs the Rust test suite.
- `just fix` runs `cargo fmt` and `cargo clippy --fix`.
- `just check` runs `cargo fmt --check` and `cargo clippy`.
