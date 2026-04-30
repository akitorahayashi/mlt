use predicates::prelude::*;

use crate::harness::TestContext;

#[test]
fn run_accepts_optional_format_flag() {
    let ctx = TestContext::new();

    ctx.command()
        .args(["run", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--format"));
}
