## Overview

This repository manages presentation source files by deck and exports Marp slide markdown through a local Rust CLI.

## Structure

```text
decks/<deck-id>/
  manuscript.md
  slides.md
  default.css
  assets/
  artifacts/

layouts/
src/
```

- `decks/<deck-id>/manuscript.md` stores the presentation narrative.
- `decks/<deck-id>/slides.md` stores the Marp-ready slide markdown.
- `decks/<deck-id>/default.css` stores the deck-local Marp theme.
- `decks/<deck-id>/assets/` stores files that belong only to that deck.
- `decks/<deck-id>/artifacts/` stores exported files for that deck.
- `layouts/` stores shared slide patterns.
- `src/assets/default.css` stores the scaffold template copied by `create`.

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

- `just run list` lists valid deck ids.
- `just run create <deck-id>` scaffolds a deck directory with `default.css`.
- `just run run <deck-id>` exports all supported formats into `decks/<deck-id>/artifacts/`.
- `just run run <deck-id> [--pdf] [--html] [--png] [--pptx]` exports selected formats.

`example-deck` is the starter deck under `decks/example-deck/`.

## Commands

```bash
just run list
just run create kyoto-go-64
just run run example-deck
just run run example-deck --pdf
just run run example-deck --html --pdf
```

## Development

- `just help` prints the available recipes.
- `just run ...` invokes the local CLI through `cargo run --`.
- `just test` runs the Rust test suite.
- `just fix` runs `cargo fmt` and `cargo clippy --fix`.
- `just check` runs `cargo fmt --check` and `cargo clippy`.
