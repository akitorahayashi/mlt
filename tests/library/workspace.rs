use marp_pj::workspace;

use crate::harness::TestContext;

#[test]
fn create_then_resolve_roundtrip() {
    let ctx = TestContext::new();

    let created = workspace::create(ctx.root(), "kyoto-go-64").expect("create deck");
    let resolved = workspace::resolve(ctx.root(), "kyoto-go-64").expect("resolve deck");

    assert_eq!(created.manifest.deck_id, "kyoto-go-64");
    assert_eq!(resolved.manifest.title, "Kyoto Go 64");
    assert_eq!(
        resolved.slides_path.file_name().unwrap_or_default(),
        "slides.md"
    );
}

#[test]
fn list_returns_only_valid_entries() {
    let ctx = TestContext::new();
    workspace::create(ctx.root(), "example-deck").expect("create valid deck");
    ctx.create_invalid_deck_dir("invalid");

    let entries = workspace::list(ctx.root()).expect("list decks");

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].id, "example-deck");
}
