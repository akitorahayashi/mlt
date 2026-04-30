## Overview

This repository manages presentation source files by deck and exports completed Marp slide markdown through a local Rust CLI.

## Structure

```text
decks/<deck-id>/
  deck.yml
  manuscript.md
  slides.md
  assets/

layouts/
themes/
src/
output/
```

- `decks/<deck-id>/manuscript.md` stores the presentation narrative.
- `decks/<deck-id>/slides.md` stores the Marp-ready slide markdown.
- `decks/<deck-id>/assets/` stores files that belong only to that deck.
- `layouts/` stores shared slide patterns.
- `themes/` stores shared Marp themes.
- `output/<deck-id>/` stores exported files.

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

- `slide-generation.md` describes how manuscripts, layouts, and themes are turned into `slides.md`.
- `just run decks list` lists valid decks.
- `just run export <format> <deck-id>` exports a managed deck.
- `just run convert <format> <slides.md> --output-dir <dir>` converts a completed slide markdown file directly.

`example-deck` is the starter deck under `decks/example-deck/`.

## Commands

```bash
just run decks list
just run decks show example-deck
just run decks create kyoto-go-64
just run export pdf example-deck
just run export all example-deck
```

The direct conversion interface is available through the CLI.

```bash
just run convert pdf decks/example-deck/slides.md --output-dir output/example-deck/direct --theme themes/default.css
```

## Development

- `just help` prints the available recipes.
- `just run ...` invokes the local CLI through `cargo run --`.
- `just test` runs the Rust test suite.
- `just fix` runs `cargo fmt` and `cargo clippy --fix`.
- `just check` runs `cargo fmt --check` and `cargo clippy`.
