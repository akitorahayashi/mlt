use predicates::prelude::*;

use crate::harness::TestContext;

#[test]
fn list_shows_only_valid_decks() {
    let ctx = TestContext::new();
    ctx.create_deck("fixture-deck");
    ctx.create_invalid_deck_dir("broken-deck");

    ctx.command()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("fixture-deck"))
        .stdout(predicate::str::contains("broken-deck").not());
}

#[test]
fn ls_alias_shows_valid_decks() {
    let ctx = TestContext::new();
    ctx.create_deck("fixture-deck");

    ctx.command()
        .arg("ls")
        .assert()
        .success()
        .stdout(predicate::str::contains("fixture-deck"));
}
