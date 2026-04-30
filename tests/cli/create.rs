use crate::harness::TestContext;
use predicates::prelude::*;

#[test]
fn create_scaffolds_default_theme_file() {
    let ctx = TestContext::new();

    ctx.command()
        .args(["create", "new-deck"])
        .assert()
        .success()
        .stdout(predicate::str::contains("decks/new-deck"));

    ctx.command()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("new-deck"));

    assert!(ctx.root().join("decks/new-deck/theme.css").is_file());
    assert!(ctx.root().join("decks/new-deck/manuscript.md").is_file());
    assert!(ctx.root().join("decks/new-deck/slides.md").is_file());
}

#[test]
fn cr_alias_creates_deck() {
    let ctx = TestContext::new();

    ctx.command()
        .args(["cr", "alias-deck"])
        .assert()
        .success()
        .stdout(predicate::str::contains("decks/alias-deck"));

    assert!(ctx.root().join("decks/alias-deck/slides.md").is_file());
}

#[test]
fn create_fails_when_deck_already_exists() {
    let ctx = TestContext::new();
    ctx.create_deck("existing-deck");

    ctx.command()
        .args(["create", "existing-deck"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Deck already exists: existing-deck",
        ));
}
