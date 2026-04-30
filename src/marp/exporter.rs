use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{AppError, AppResult};

use super::Format;

const CUSTOM_CSS_IMPORT_RULE: &str = "@import 'custom.css';";
const EXPORT_THEME_FILENAME: &str = ".marp-pj-theme.css";

pub fn export_many(
    slides_path: &Path,
    output_dir: &Path,
    basename: &str,
    theme: Option<&Path>,
    formats: &[Format],
) -> AppResult<Vec<PathBuf>> {
    let mut exported = Vec::new();
    for format in formats {
        exported.push(export(slides_path, output_dir, basename, theme, format)?);
    }
    Ok(exported)
}

fn export(
    slides_path: &Path,
    output_dir: &Path,
    basename: &str,
    theme: Option<&Path>,
    format: &Format,
) -> AppResult<PathBuf> {
    ensure_exists("Slides", slides_path)?;
    std::fs::create_dir_all(output_dir)?;
    let export_theme = materialize_theme(theme, output_dir)?;
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

fn materialize_theme(theme: Option<&Path>, output_dir: &Path) -> AppResult<Option<PathBuf>> {
    let Some(theme_path) = theme else {
        return Ok(None);
    };
    ensure_exists("Theme", theme_path)?;

    let theme_css = std::fs::read_to_string(theme_path)?;
    if !theme_css.contains(CUSTOM_CSS_IMPORT_RULE) {
        return Ok(Some(theme_path.to_path_buf()));
    };

    let Some(theme_dir) = theme_path.parent() else {
        return Err(AppError::MissingPath {
            kind: "Theme directory",
            path: theme_path.to_path_buf(),
        });
    };

    let custom_css_path = theme_dir.join("custom.css");
    ensure_exists("Custom theme", &custom_css_path)?;

    let custom_css = std::fs::read_to_string(&custom_css_path)?;
    let export_theme_path = output_dir.join(EXPORT_THEME_FILENAME);
    let theme_css_without_custom_import = theme_css.replace(CUSTOM_CSS_IMPORT_RULE, "");
    std::fs::write(
        &export_theme_path,
        format!("{theme_css_without_custom_import}\n{custom_css}"),
    )?;
    Ok(Some(export_theme_path))
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
    fn materialize_theme_inlines_deck_custom_css() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let deck_dir = temp_dir.path().join("deck");
        let output_dir = temp_dir.path().join("artifacts");
        std::fs::create_dir_all(&deck_dir).expect("deck dir");
        std::fs::create_dir_all(&output_dir).expect("output dir");

        let theme_path = deck_dir.join("default.css");
        std::fs::write(
            &theme_path,
            "/* @theme marp-pj-default */\n@import 'default';\n@import 'custom.css';\nsection { color: white; }\n",
        )
        .expect("theme css");
        std::fs::write(
            deck_dir.join("custom.css"),
            "section pre .hljs-keyword { color: #ff7b72; }\n",
        )
        .expect("custom css");

        let export_theme = materialize_theme(Some(&theme_path), &output_dir)
            .expect("materialize theme")
            .expect("theme path");
        let export_css = std::fs::read_to_string(export_theme).expect("export theme css");

        assert!(!export_css.contains(CUSTOM_CSS_IMPORT_RULE));
        assert!(export_css.contains("@import 'default';"));
        assert!(export_css.contains("section pre .hljs-keyword { color: #ff7b72; }"));
        assert!(
            export_css.find("section { color: white; }")
                < export_css.find("section pre .hljs-keyword")
        );
    }

    #[test]
    fn materialize_theme_requires_imported_custom_css() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let deck_dir = temp_dir.path().join("deck");
        let output_dir = temp_dir.path().join("artifacts");
        std::fs::create_dir_all(&deck_dir).expect("deck dir");
        std::fs::create_dir_all(&output_dir).expect("output dir");

        let theme_path = deck_dir.join("default.css");
        std::fs::write(
            &theme_path,
            "/* @theme marp-pj-default */\n@import 'custom.css';\nsection { color: white; }\n",
        )
        .expect("theme css");

        let error = materialize_theme(Some(&theme_path), &output_dir).expect_err("missing custom");

        assert!(matches!(
            error,
            AppError::MissingPath {
                kind: "Custom theme",
                ..
            }
        ));
    }
}
