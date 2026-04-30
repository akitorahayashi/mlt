use predicates::prelude::*;
use std::fs;

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

    assert!(ctx.root().join("decks/new-deck/default.css").is_file());
    assert!(ctx.root().join("decks/new-deck/theme.css").is_file());
    assert!(ctx.root().join("decks/new-deck/css/canvas.css").is_file());
    assert!(ctx.root().join("decks/new-deck/css/code.css").is_file());
    let css_content =
        fs::read_to_string(ctx.root().join("decks/new-deck/default.css")).expect("read css");
    assert!(css_content.contains("@theme marp-pj-default"));
    assert!(css_content.contains("@import 'theme.css';"));
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
