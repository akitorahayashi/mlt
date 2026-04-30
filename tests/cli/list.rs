use predicates::prelude::*;

use crate::harness::TestContext;

#[test]
fn list_shows_only_valid_decks() {
    let ctx = TestContext::new();
    ctx.create_deck("example-deck");
    ctx.create_invalid_deck_dir("broken-deck");

    ctx.command()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("example-deck"))
        .stdout(predicate::str::contains("broken-deck").not());
}
