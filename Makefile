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

# Marp slide generation settings (implementation-agnostic interface)
SLIDES_SRC := ./output/slides.md
THEME_PATH := ./src/theme.css
OUTPUT_DIR := ./output

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
pdf: ## Generate PDF from output/slides.md using Marp
	@echo "📄 Generating PDF from $(SLIDES_SRC)..."
	@marp $(SLIDES_SRC) --theme $(THEME_PATH) -o $(OUTPUT_DIR)/slides.pdf

.PHONY: pptx
pptx: ## Generate PPTX from output/slides.md using Marp
	@echo "📊 Generating PPTX from $(SLIDES_SRC)..."
	@marp $(SLIDES_SRC) --theme $(THEME_PATH) -o $(OUTPUT_DIR)/slides.pptx

.PHONY: html
html: ## Generate HTML from output/slides.md using Marp
	@echo "🌐 Generating HTML from $(SLIDES_SRC)..."
	@marp $(SLIDES_SRC) --theme $(THEME_PATH) -o $(OUTPUT_DIR)/slides.html

.PHONY: all
all: ## Generate PDF, PPTX, and HTML from output/slides.md
	@$(MAKE) pdf
	@$(MAKE) pptx
	@$(MAKE) html

# ==============================================================================
# CODE QUALITY
# ==============================================================================

.PHONY: format
format: ## Automatically format code using Black and Ruff
	@echo "🎨 Formatting code with black and ruff..."
	@uv run black .
	@uv run ruff check . --fix

.PHONY: lint
lint: ## Perform static code analysis (check) using Black and Ruff
	@echo "🔬 Linting code with black and ruff..."
	@uv run black --check .
	@uv run ruff check .

# ==============================================================================
# TESTING
# ==============================================================================

.PHONY: test
test: ## Run the full test suite
	@echo "Running unit tests..."
	@uv run pytest -v -s
