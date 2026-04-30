use predicates::prelude::*;

use crate::harness::TestContext;

#[test]
fn help_lists_primary_commands() {
    let ctx = TestContext::new();

    ctx.command()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("decks"))
        .stdout(predicate::str::contains("export"))
        .stdout(predicate::str::contains("convert"));
}
