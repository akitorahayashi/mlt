use predicates::prelude::*;

use crate::harness::TestContext;

#[test]
fn run_accepts_per_format_flags() {
    let ctx = TestContext::new();

    ctx.command()
        .args(["run", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--pdf"))
        .stdout(predicate::str::contains("--html"))
        .stdout(predicate::str::contains("--pptx"))
        .stdout(predicate::str::contains("--png").not())
        .stdout(predicate::str::contains("--format").not());
}

#[test]
fn r_alias_accepts_per_format_flags() {
    let ctx = TestContext::new();

    ctx.command()
        .args(["r", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--pdf"))
        .stdout(predicate::str::contains("--html"))
        .stdout(predicate::str::contains("--pptx"))
        .stdout(predicate::str::contains("--png").not());
}
