use predicates::prelude::*;

use crate::harness::TestContext;

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
}
