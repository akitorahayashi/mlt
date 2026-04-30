use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{AppError, AppResult};

use super::Format;

const DEFAULT_THEME_CSS: &str = include_str!("../assets/default.css");
const CANVAS_CSS: &str = include_str!("../assets/css/canvas.css");
const HEADING_CSS: &str = include_str!("../assets/css/heading.css");
const LIST_CSS: &str = include_str!("../assets/css/list.css");
const HIGHLIGHT_CSS: &str = include_str!("../assets/css/highlight.css");
const CODE_CSS: &str = include_str!("../assets/css/code.css");
const EXPORT_THEME_FILENAME: &str = ".marp-pj-theme.css";

pub fn export_many(
    slides_path: &Path,
    output_dir: &Path,
    basename: &str,
    theme: Option<&Path>,
    formats: &[Format],
) -> AppResult<Vec<PathBuf>> {
    std::fs::create_dir_all(output_dir)?;
    let export_theme = materialize_theme(theme, output_dir)?;
    let mut exported = Vec::new();
    for format in formats {
        exported.push(export(
            slides_path,
            output_dir,
            basename,
            &export_theme,
            format,
        )?);
    }
    Ok(exported)
}

fn export(
    slides_path: &Path,
    output_dir: &Path,
    basename: &str,
    export_theme: &Option<PathBuf>,
    format: &Format,
) -> AppResult<PathBuf> {
    ensure_exists("Slides", slides_path)?;
    let output_path = output_dir.join(format!("{basename}.{}", (*format).extension()));

    let mut command = Command::new("marp");
    command
        .arg(slides_path)
        .arg("--allow-local-files")
        .arg("-o")
        .arg(&output_path);
    if let Some(theme_path) = export_theme {
        command.arg("--theme").arg(theme_path);
    }

    let output = command.output().map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            AppError::MarpCommandFailed(
                "Marp CLI ('marp' command) not found in PATH. Please install it via 'npm install -g @marp-team/marp-cli'."
                    .to_string(),
            )
        } else {
            AppError::Io(error)
        }
    })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let detail = if !stderr.is_empty() {
            stderr
        } else if !stdout.is_empty() {
            stdout
        } else {
            format!("marp exited with status {}", output.status)
        };
        return Err(AppError::MarpCommandFailed(detail));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.trim().is_empty() {
        print!("{stdout}");
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.trim().is_empty() {
        eprint!("{stderr}");
    }

    Ok(output_path)
}

pub fn materialize_theme(theme: Option<&Path>, output_dir: &Path) -> AppResult<Option<PathBuf>> {
    let Some(theme_override_path) = theme else {
        return Ok(None);
    };
    ensure_exists("Theme", theme_override_path)?;

    let export_theme_path = output_dir.join(EXPORT_THEME_FILENAME);
    let mut import_stack = Vec::new();
    let expanded_theme_css = expand_shared_theme(theme_override_path, &mut import_stack)?;
    std::fs::write(&export_theme_path, expanded_theme_css)?;
    Ok(Some(export_theme_path))
}

fn expand_shared_theme(
    theme_override_path: &Path,
    import_stack: &mut Vec<PathBuf>,
) -> AppResult<String> {
    let mut expanded = String::new();

    for line in DEFAULT_THEME_CSS.lines() {
        if let Some(import_target) = parse_import_target(line) {
            match import_target {
                "default" => {
                    expanded.push_str(line);
                    expanded.push('\n');
                }
                "theme.css" => {
                    expanded.push_str(&expand_theme_css(theme_override_path, import_stack)?);
                    if !expanded.ends_with('\n') {
                        expanded.push('\n');
                    }
                }
                asset_path => {
                    let asset_css = shared_theme_asset(asset_path).ok_or_else(|| {
                        AppError::ThemeCssImportFailed(format!(
                            "unknown shared theme asset import: {asset_path}"
                        ))
                    })?;
                    expanded.push_str(asset_css);
                    if !asset_css.ends_with('\n') {
                        expanded.push('\n');
                    }
                }
            }
            continue;
        }

        expanded.push_str(line);
        expanded.push('\n');
    }

    Ok(expanded)
}

fn shared_theme_asset(path: &str) -> Option<&'static str> {
    match path {
        "css/canvas.css" => Some(CANVAS_CSS),
        "css/heading.css" => Some(HEADING_CSS),
        "css/list.css" => Some(LIST_CSS),
        "css/highlight.css" => Some(HIGHLIGHT_CSS),
        "css/code.css" => Some(CODE_CSS),
        _ => None,
    }
}

fn expand_theme_css(css_path: &Path, import_stack: &mut Vec<PathBuf>) -> AppResult<String> {
    let canonical_path = std::fs::canonicalize(css_path)?;
    if let Some(position) = import_stack.iter().position(|path| path == &canonical_path) {
        let cycle = import_stack[position..]
            .iter()
            .chain(std::iter::once(&canonical_path))
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(" -> ");
        return Err(AppError::ThemeCssImportFailed(format!(
            "cyclic import chain: {cycle}"
        )));
    }

    import_stack.push(canonical_path.clone());
    let css = std::fs::read_to_string(&canonical_path)?;
    let base_dir = canonical_path.parent().ok_or_else(|| {
        AppError::ThemeCssImportFailed(format!(
            "unable to resolve parent directory for {}",
            canonical_path.display()
        ))
    })?;
    let mut expanded = String::new();

    for line in css.lines() {
        if let Some(import_target) = parse_import_target(line) {
            let imported_path = base_dir.join(import_target);
            ensure_exists("Theme import", &imported_path)?;
            expanded.push_str(&expand_theme_css(&imported_path, import_stack)?);
            if !expanded.ends_with('\n') {
                expanded.push('\n');
            }
            continue;
        }

        expanded.push_str(line);
        expanded.push('\n');
    }

    import_stack.pop();
    Ok(expanded)
}

fn parse_import_target(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if !trimmed.starts_with("@import") {
        return None;
    }
    let remainder = trimmed["@import".len()..].trim_start();
    let quote = if remainder.starts_with('\'') {
        '\''
    } else if remainder.starts_with('"') {
        '"'
    } else {
        return None;
    };
    let remainder = &remainder[1..];
    let end = remainder.find(quote)?;
    let target = &remainder[..end];
    let after_quote = remainder[end + 1..].trim();
    if after_quote.starts_with(';') {
        Some(target)
    } else {
        None
    }
}

fn ensure_exists(kind: &'static str, path: &Path) -> AppResult<()> {
    if path.exists() {
        return Ok(());
    }
    Err(AppError::MissingPath {
        kind,
        path: path.to_path_buf(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn materialize_theme_inlines_local_css_imports() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let deck_dir = temp_dir.path().join("deck");
        let output_dir = temp_dir.path().join("artifacts");
        std::fs::create_dir_all(&deck_dir).expect("deck dir");
        std::fs::create_dir_all(&output_dir).expect("output dir");

        let theme_path = deck_dir.join("theme.css");
        std::fs::write(deck_dir.join("extra.css"), "section { color: #111111; }\n")
            .expect("extra css");
        std::fs::write(
            &theme_path,
            "@import 'extra.css';\nsection { letter-spacing: 0; }\n",
        )
        .expect("theme css");

        let export_theme = materialize_theme(Some(&theme_path), &output_dir)
            .expect("materialize theme")
            .expect("theme path");
        let export_css = std::fs::read_to_string(export_theme).expect("export theme css");

        assert!(export_css.contains("@import 'default';"));
        assert!(export_css.contains(":is(pre, marp-pre) .hljs-keyword"));
        assert!(export_css.contains("section { color: #111111; }"));
        assert!(export_css.contains("section { letter-spacing: 0; }"));
        assert!(
            export_css.find(":is(pre, marp-pre) .hljs-keyword")
                < export_css.find("section { color: #111111; }")
        );
    }

    #[test]
    fn materialize_theme_requires_imported_theme_file() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let deck_dir = temp_dir.path().join("deck");
        let output_dir = temp_dir.path().join("artifacts");
        std::fs::create_dir_all(&deck_dir).expect("deck dir");
        std::fs::create_dir_all(&output_dir).expect("output dir");

        let theme_path = deck_dir.join("theme.css");
        std::fs::write(&theme_path, "@import 'missing.css';\n").expect("theme css");

        let error = materialize_theme(Some(&theme_path), &output_dir).expect_err("missing theme");

        assert!(matches!(
            error,
            AppError::MissingPath {
                kind: "Theme import",
                ..
            }
        ));
    }
}
