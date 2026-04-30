use crate::harness::TestContext;

#[test]
fn materialized_theme_contains_custom_css_imports() {
    let ctx = TestContext::new();
    ctx.create_deck("fixture-deck");

    let deck_dir = ctx.root().join("decks").join("fixture-deck");
    let extra_css = deck_dir.join("extra.css");
    let theme_path = deck_dir.join("theme.css");
    std::fs::write(&extra_css, "section { color: #abcdef; }\n").expect("write extra css");
    std::fs::write(
        &theme_path,
        "@import 'extra.css';\nsection { letter-spacing: 1; }\n",
    )
    .expect("write theme css");

    let output_dir = tempfile::TempDir::new().expect("tempdir");
    let output_path = output_dir.path();

    let export = mlt::marp::materialize_theme(Some(&theme_path), output_path)
        .expect("materialize theme")
        .expect("theme path returned");
    let css = std::fs::read_to_string(export).expect("read export css");

    assert!(css.contains("section { color: #abcdef; }"));
    assert!(css.contains("section { letter-spacing: 1; }"));
}
