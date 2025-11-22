## Overview

A Python project for creating presentation slides using Marp CLI.

## Usage

### Prerequisites

- Python 3.12+
- [uv](https://github.com/astral-sh/uv)
- [marp-cli](https://github.com/marp-team/marp-cli)

You can install `marp-cli` with Homebrew or npm.

```bash
# npm
npm install -g @marp-team/marp-cli
```

### Setup

Install project dependencies.

```bash
make setup
```

### Workflow Overview

This project is designed so that an AI agent (e.g., Codex) generates the actual slide deck.

```bash
# 1. You write the manuscript
edit src/script.md

# 2. AI agent reads:
#    - src/prompt.md   (instructions)
#    - src/script.md   (manuscript)
#    - src/layouts/    (layout library: one layout per file)
#    and writes the deck to:
output/slides.md

# 3. You export slides with Marp via Makefile targets
make pdf   # output/slides.pdf
make pptx  # output/slides.pptx
make html  # output/slides.html
```

The visual style is defined in `src/theme.css` and registered as the `custom-theme` theme, aligned with the `2025-business-pitch-deck` project.

### Makefile Targets

- `make pdf`: Generate `output/slides.pdf` from `output/slides.md` using Marp.
- `make pptx`: Generate `output/slides.pptx`.
- `make html`: Generate `output/slides.html`.
- `make all`: Generate PDF, PPTX, and HTML in one shot.

### Other Commands

- `make help`: Show all available commands.
- `make test`: Run tests.
- `make format`: Format code.
- `make lint`: Run linter.
