---
title: Testing Patterns in Rust
language: en
---

This manuscript covers practical testing patterns used in Rust projects.

## Unit Testing Fundamentals

Unit tests validate individual functions in isolation. The `#[test]` attribute marks test functions, and `assert!` macros verify expected behavior.

Example: Testing a validation function that returns an error for invalid input strings requires checking both the Ok path and Err path separately.

## Integration Testing Strategy

Integration tests verify that modules work correctly together. They live in the `tests/` directory and have access to the crate's public API.

Key consideration: Integration tests should not freeze internal implementation details. They assert observable behavior at boundaries, not internal composition.

## Test Fixtures and Temporary Files

Tests that need temporary state use `tempfile::TempDir` to create isolated scratch space. This prevents tests from interfering with one another and ensures cleanup.

Best practice: Keep temporary operations confined to the project root or designated test directories.

## CSS Materialization in Presentation Tools

When exporting presentations, CSS files must be correctly materialized—that is, imported dependencies must be inlined or correctly referenced so that styles appear in the final artifact.

Test validation: Assert that materialized CSS output contains expected style declarations from both the base theme and deck-specific overrides.
