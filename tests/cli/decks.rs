use predicates::prelude::*;

use crate::harness::TestContext;

#[test]
fn list_shows_only_valid_decks() {
    let ctx = TestContext::new();
    ctx.create_deck("example-deck");
    ctx.create_invalid_deck_dir("broken-deck");

    ctx.command()
        .args(["decks", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("example-deck\tExample Deck"))
        .stdout(predicate::str::contains("broken-deck").not());
}

#[test]
fn show_prints_resolved_paths() {
    let ctx = TestContext::new();
    ctx.create_deck("example-deck");

    ctx.command()
        .args(["decks", "show", "example-deck"])
        .assert()
        .success()
        .stdout(predicate::str::contains("deck-id: example-deck"))
        .stdout(predicate::str::contains("manuscript.md"))
        .stdout(predicate::str::contains("slides.md"))
        .stdout(predicate::str::contains("default.css"));
}
