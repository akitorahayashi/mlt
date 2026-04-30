# ==============================================================================
# justfile for deck management and Marp export
# ==============================================================================

default: help

default_deck := "example-deck"
default_output_basename := "slides"

help:
    @echo "Usage: just [recipe]"
    @echo "Available recipes:"
    @just --list | tail -n +2 | awk '{printf "  \033[36m%-20s\033[0m %s\n", $1, substr($0, index($0, $2))}'

# Install Python dependencies
setup:
    @echo "Installing Python dependencies with uv..."
    @uv sync

# Export PDF for a managed deck
pdf deck=default_deck:
    @uv run python -m src.cli export {{deck}} --to pdf

# Export HTML for a managed deck
html deck=default_deck:
    @uv run python -m src.cli export {{deck}} --to html

# Export PNG for a managed deck
png deck=default_deck:
    @uv run python -m src.cli export {{deck}} --to png

# Export PPTX for a managed deck
pptx deck=default_deck:
    @uv run python -m src.cli export {{deck}} --to pptx

# Export all supported formats for a managed deck
all deck=default_deck:
    @uv run python -m src.cli export {{deck}} --to all

# Convert a completed slides.md directly
convert slides_path output_dir format theme_file="" output_basename=default_output_basename:
    @if [ -n "{{theme_file}}" ]; then \
        uv run python -m src.cli convert "{{slides_path}}" --output-dir "{{output_dir}}" --theme-file "{{theme_file}}" --output-basename "{{output_basename}}" --to "{{format}}"; \
    else \
        uv run python -m src.cli convert "{{slides_path}}" --output-dir "{{output_dir}}" --output-basename "{{output_basename}}" --to "{{format}}"; \
    fi

# Automatically format and fix project code
fix:
    @echo "Formatting and fixing project code..."
    @uv run black src tests
    @uv run ruff check src tests --fix

# Run static checks for project code
check:
    @echo "Running static checks..."
    @uv run black --check src tests
    @uv run ruff check src tests

# Run repository tests
test:
    @echo "Running unit tests..."
    @uv run pytest -q
