use predicates::prelude::*;

use crate::harness::TestContext;

#[test]
fn help_lists_primary_commands() {
    let ctx = TestContext::new();

    ctx.command()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("cr"))
        .stdout(predicate::str::contains("run"))
        .stdout(predicate::str::contains("r"));
}

#[test]
fn version_prints_app_version() {
    let ctx = TestContext::new();

    ctx.command()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}
