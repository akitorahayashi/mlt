use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{AppError, AppResult};

use super::Format;

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
    if let Some(theme_path) = theme {
        ensure_exists("Theme", theme_path)?;
    }

    std::fs::create_dir_all(output_dir)?;
    let output_path = output_dir.join(format!("{basename}.{}", (*format).extension()));

    let mut command = Command::new("marp");
    command
        .arg(slides_path)
        .arg("--allow-local-files")
        .arg("-o")
        .arg(&output_path);
    if let Some(theme_path) = theme {
        command.arg("--theme").arg(theme_path);
    }

    let output = command.output()?;
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

    Ok(output_path)
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
