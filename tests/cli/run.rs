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
        .stdout(predicate::str::contains("--png"))
        .stdout(predicate::str::contains("--pptx"))
        .stdout(predicate::str::contains("--format").not());
}
