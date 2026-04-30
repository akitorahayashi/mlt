# ==============================================================================
# Makefile for Project Automation
#
# Provides a unified interface for common development tasks, such as running
# the application, formatting code, and running tests.
#
# Inspired by the self-documenting Makefile pattern.
# See: https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
# ==============================================================================

# Default target when 'make' is run without arguments
.DEFAULT_GOAL := help

DECK ?= example-deck

# ==============================================================================
# HELP
# ==============================================================================

.PHONY: help 
help: ## Show this help message
	@echo "Usage: make [target]"
	@echo "Available targets:"
	@awk 'BEGIN {FS = ":.*?## "} /^[^_][a-zA-Z0-9_-]*:.*?## / {printf "  \033[36m%-25s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# ==============================================================================
# ENVIRONMENT SETUP
# ==============================================================================

.PHONY: setup
setup: ## Project initial setup: install dependencies and create .env file
	@echo "🐍 Installing python dependencies with uv..."
	@uv sync

# ==============================================================================
# SLIDE EXPORTS (via Marp CLI)
# ==============================================================================

.PHONY: pdf
pdf: ## Generate PDF for DECK=<deck-id>
	@uv run python -m src.cli export $(DECK) --to pdf

.PHONY: pptx
pptx: ## Generate PPTX for DECK=<deck-id>
	@uv run python -m src.cli export $(DECK) --to pptx

.PHONY: html
html: ## Generate HTML for DECK=<deck-id>
	@uv run python -m src.cli export $(DECK) --to html

.PHONY: png
png: ## Generate PNG for DECK=<deck-id>
	@uv run python -m src.cli export $(DECK) --to png

.PHONY: all
all: ## Generate PDF, HTML, PNG, and PPTX for DECK=<deck-id>
	@uv run python -m src.cli export $(DECK) --to all

# ==============================================================================
# CODE QUALITY
# ==============================================================================

.PHONY: format
format: ## Automatically format code using Black and Ruff
	@echo "🎨 Formatting code with black and ruff..."
	@uv run black src tests
	@uv run ruff check src tests --fix

.PHONY: lint
lint: ## Perform static code analysis (check) using Black and Ruff
	@echo "🔬 Linting code with black and ruff..."
	@uv run black --check src tests
	@uv run ruff check src tests

# ==============================================================================
# TESTING
# ==============================================================================

.PHONY: test
test: ## Run the full test suite
	@echo "Running unit tests..."
	@uv run pytest -q
