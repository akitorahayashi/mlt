## Overview

This repository manages presentation source files by deck and exports completed Marp slide markdown through a dedicated conversion CLI.

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

- Python 3.12+
- [uv](https://github.com/astral-sh/uv)
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
- `python -m src.cli export <deck-id> --to <format>` exports a managed deck.
- `python -m src.cli convert <slides.md> --output-dir <dir> --to <format>` converts a completed slide markdown file directly.

`example-deck` is the starter deck under `decks/example-deck/`.

## Commands

```bash
just pdf example-deck
just html example-deck
just png example-deck
just pptx example-deck
just all example-deck
```

The direct conversion interface is available through the CLI.

```bash
just convert decks/example-deck/slides.md output/example-deck/direct pdf themes/default.css
```

## Development

- `just help` prints the available recipes.
- `just test` runs the repository test suite.
- `just fix` runs Black and Ruff fixes.
- `just check` runs Black and Ruff checks.
