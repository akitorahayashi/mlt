use std::path::PathBuf;

#[test]
fn fixture_deck_resolves_correctly() {
    let fixture_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/test-fixture-deck");

    let deck_layout = mlt::deck_layout::resolve_dir(&fixture_path).expect("fixture should resolve");

    assert_eq!(deck_layout.deck_id, "test-fixture-deck");
    assert!(deck_layout.manuscript_path.is_file());
    assert!(deck_layout.slides_path.is_file());
    assert!(deck_layout.theme_path.is_file());
    assert!(deck_layout.artifacts_dir.is_dir());
}

#[test]
fn fixture_theme_materializes_with_custom_styles() {
    let fixture_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/test-fixture-deck");

    let theme_path = fixture_path.join("theme.css");
    let output_dir = tempfile::TempDir::new().expect("temp dir");

    let export = mlt::marp::materialize_theme(Some(&theme_path), output_dir.path())
        .expect("materialize fixture theme")
        .expect("export theme path returned");

    let css = std::fs::read_to_string(export).expect("read materialized CSS");

    // Validate that fixture-specific CSS is present
    assert!(
        css.contains("section.title-slide"),
        "materialized CSS should contain fixture title-slide style"
    );
    assert!(
        css.contains("#667eea"),
        "materialized CSS should contain fixture color variable"
    );
    assert!(
        css.contains("border-left: 3px solid #667eea"),
        "materialized CSS should contain fixture code styling"
    );
    assert!(
        css.contains("border-bottom: 2px solid #667eea"),
        "materialized CSS should contain fixture h2 styling"
    );

    // Validate that shared theme is still present
    assert!(
        css.contains("/* @theme mlt-default */"),
        "materialized CSS should reference default theme"
    );
}

#[test]
fn fixture_exports_all_formats_with_materialized_theme() {
    let fixture_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/test-fixture-deck");

    let deck_layout = mlt::deck_layout::resolve_dir(&fixture_path).expect("fixture should resolve");

    // Export all formats (HTML, PDF, PPTX) to fixture artifacts directory
    let export_artifacts = mlt::marp::export_many(
        &deck_layout.slides_path,
        &deck_layout.artifacts_dir,
        "slides",
        Some(&deck_layout.theme_path),
        &[
            mlt::marp::Format::Html,
            mlt::marp::Format::Pdf,
            mlt::marp::Format::Pptx,
        ],
    )
    .expect("export to all formats should succeed");

    assert_eq!(
        export_artifacts.len(),
        3,
        "should export exactly three files (HTML, PDF, PPTX)"
    );

    // Verify all three files exist and have reasonable sizes
    let mut html_path = None;
    let mut pdf_path = None;
    let mut pptx_path = None;

    for artifact_path in &export_artifacts {
        assert!(
            artifact_path.exists(),
            "exported file should exist at {}",
            artifact_path.display()
        );

        let metadata = std::fs::metadata(artifact_path).expect("should read file metadata");
        let file_size = metadata.len();
        assert!(
            file_size > 0,
            "exported file {} should have content (size: {})",
            artifact_path.display(),
            file_size
        );

        match artifact_path.extension().and_then(|e| e.to_str()) {
            Some("html") => html_path = Some(artifact_path.clone()),
            Some("pdf") => pdf_path = Some(artifact_path.clone()),
            Some("pptx") => pptx_path = Some(artifact_path.clone()),
            _ => panic!("unexpected file extension: {:?}", artifact_path),
        }
    }

    // Verify all three formats were created
    assert!(html_path.is_some(), "HTML file should be created");
    assert!(pdf_path.is_some(), "PDF file should be created");
    assert!(pptx_path.is_some(), "PPTX file should be created");

    // Validate HTML content
    let html_content = std::fs::read_to_string(html_path.as_ref().unwrap())
        .expect("should read exported HTML file");

    assert!(
        html_content.contains("Testing Patterns in Rust") || html_content.contains("title-slide"),
        "HTML should contain slide title or styles"
    );
    assert!(
        html_content.contains("1600x900.png") || html_content.contains("img"),
        "HTML should contain image reference"
    );
}
