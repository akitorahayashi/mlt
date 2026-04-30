# Goal

The repository stores presentation manuscripts and completed Marp slide decks separately.

# Inputs

- Manuscript: `decks/<deck-id>/manuscript.md`
- Layout library: `layouts/`
- Theme library: `themes/`
- Deck metadata: `decks/<deck-id>/deck.yml`

# Output

- Completed slide markdown: `decks/<deck-id>/slides.md`

# Style

- The layout patterns in `layouts/` are the primary slide building blocks.
- The theme in `themes/default.css` defines the shared visual language.
- Each slide is centered on one idea and avoids paragraph-heavy output.

# Deck Boundaries

- `manuscript.md` stores the presentation narrative and speaking material.
- `slides.md` stores the Marp-ready slide source for export.
- `assets/` stores files that belong only to that deck.

# Deck Workflow

1. Read `decks/<deck-id>/manuscript.md`.
2. Select the appropriate patterns from `layouts/`.
3. Write the completed Marp deck to `decks/<deck-id>/slides.md`.
4. Export the deck through `python -m src.cli export <deck-id> --to <format>`.
