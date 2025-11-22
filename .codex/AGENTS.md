# marp-pj Integration Notes for AI Agents

Scope: This file describes how AI agents should interact with the `marp-pj` project to generate slides. It does not define business logic or implementation details.

## Project Role
- `marp-pj` is a Marp-based slide generator tailored for a 2025-style business pitch deck.
- AI agents are responsible for creating the actual slide deck markdown; the project provides theme, layouts, and export commands.

## Key Files
- `marp-pj/src/script.md`
  - Manuscript written by the human author (full narrative, free-form).
  - Treat this as the single source of truth for content.
- `marp-pj/src/prompt.md`
  - Canonical instructions for how AI agents should read the manuscript and generate slides.
  - Follow its sections strictly (Goal, Inputs, Output format, Style guide, Layout rules, Do/Do-not, Workflow).
- `marp-pj/src/layouts/`
  - Directory of reusable Marp slide layouts with placeholders (e.g., `{{TITLE}}`, `{{KEY_METRIC}}`).
  - Layouts are organized by story phase:
    - `intro/` (cover, agenda, dividers)
    - `problem-solution/` (problems, key idea, solution)
    - `proof/` (use cases, capabilities, traction)
    - `execution/` (roadmap, process)
    - `business/` (cost structure, business model)
    - `closing/` (summary, ask)
    - `profile/` (team / profile visuals)
  - Each file has a brief comment header describing `category`, `purpose`, and `style`. Prefer using and adapting these layouts over inventing new ad-hoc structures.
- `marp-pj/src/theme.css`
  - `custom-theme` aligned with the `2025-business-pitch-deck` design.
  - Do not change the theme name; refer to it as `custom-theme` in front matter.
- `marp-pj/output/slides.md`
  - Target file AI agents must write.
  - Must contain valid Marp markdown with appropriate front matter.

## Expected Workflow for AI Agents
1. Read `marp-pj/src/prompt.md` to understand the rules and output format.
2. Read `marp-pj/src/script.md` completely to understand the story and structure (do not assume a fixed outline; infer sections).
3. Use the files under `marp-pj/src/layouts/` to choose slide layouts and fill placeholders from the manuscript, selecting layouts whose `category` and `purpose` match the current story phase.
4. Generate a coherent deck in `marp-pj/output/slides.md` with:
   - `marp: true`
   - `theme: custom-theme`
   - `paginate: true`
   - Appropriate `header` / `footer` for a business pitch.
5. Avoid decks made only of bullet-list slides; mix narrative, highlight, comparison, and visual-first layouts.

## Build / Export Interface
- AI agents should not run Marp directly; they only write `output/slides.md`.
- Humans (or CI) will run the Makefile targets in `marp-pj`:
  - `make pdf` → `output/slides.pdf`
  - `make pptx` → `output/slides.pptx`
  - `make html` → `output/slides.html`
  - `make all` → all three formats

## When the User Requests Structural Changes
- If the user asks to change the fundamental slide structure (for example, “add a new type of roadmap slide” or “change how the problem slide is framed”) rather than just changing the text:
  - Prefer updating or extending the templates under `marp-pj/src/layouts/`:
    - Modify the relevant layout file (e.g., `problem-solution/problem-highlight.md`) when the change is localized.
    - Add a new layout file in the appropriate subdirectory when introducing a new pattern.
  - Keep `output/slides.md` as a generated artifact; treat structural feedback as a reason to evolve the layout library, not to hardcode one-off structures directly in the output.
