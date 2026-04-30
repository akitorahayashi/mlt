use mlt::deck_layout;

use crate::harness::TestContext;

#[test]
fn resolve_dir_accepts_valid_deck_directory() {
    let ctx = TestContext::new();
    ctx.create_deck("kyoto-go-64");

    let deck_dir = ctx.root().join("decks").join("kyoto-go-64");
    let resolved = deck_layout::resolve_dir(&deck_dir).expect("resolve deck layout");

    assert_eq!(resolved.deck_id, "kyoto-go-64");
    assert_eq!(
        resolved.slides_path.file_name().unwrap_or_default(),
        "slides.md"
    );
    assert_eq!(
        resolved.theme_path.file_name().unwrap_or_default(),
        "theme.css"
    );
}

#[test]
fn resolve_dir_rejects_incomplete_directory() {
    let ctx = TestContext::new();
    let incomplete = ctx.root().join("decks").join("incomplete");
    std::fs::create_dir_all(&incomplete).expect("create incomplete deck");

    let result = deck_layout::resolve_dir(&incomplete);
    assert!(result.is_err());
}
