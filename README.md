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

An npm installation of Marp CLI is compatible with the current commands.

```bash
npm install -g @marp-team/marp-cli
```

## Setup

```bash
make setup
```

## Deck Workflow

The repository keeps manuscript management and slide conversion separate.

- `slide-generation.md` describes how manuscripts, layouts, and themes are turned into `slides.md`.
- `python -m src.cli export <deck-id> --to <format>` exports a managed deck.
- `python -m src.cli convert <slides.md> --output-dir <dir> --to <format>` converts a completed slide markdown file directly.

`example-deck` is the starter deck under `decks/example-deck/`.

## Commands

```bash
make pdf DECK=example-deck
make html DECK=example-deck
make png DECK=example-deck
make pptx DECK=example-deck
make all DECK=example-deck
```

The direct conversion interface is available through the CLI.

```bash
uv run python -m src.cli convert decks/example-deck/slides.md \
  --output-dir output/example-deck/direct \
  --theme-file themes/default.css \
  --to pdf
```

## Development

- `make help` prints the available targets.
- `make test` runs the repository test suite.
- `make format` runs Black and Ruff fixes.
- `make lint` runs Black and Ruff checks.
