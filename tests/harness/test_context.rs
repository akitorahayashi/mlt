use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use tempfile::TempDir;

pub struct TestContext {
    _temp_dir: TempDir,
    root: PathBuf,
}

#[allow(dead_code)]
impl TestContext {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("temp dir");
        let root = temp_dir.path().join("workspace");
        create_workspace(&root);
        Self {
            _temp_dir: temp_dir,
            root,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn create_deck(&self, id: &str) {
        let deck_dir = self.root.join("decks").join(id);
        fs::create_dir_all(deck_dir.join("assets")).expect("create deck assets");
        fs::create_dir_all(deck_dir.join("artifacts")).expect("create deck artifacts");
        fs::write(deck_dir.join("assets").join(".gitkeep"), "").expect("seed assets gitkeep");
        fs::write(deck_dir.join("artifacts").join(".gitkeep"), "").expect("seed artifacts gitkeep");
        fs::write(deck_dir.join("manuscript.md"), "# Test\n").expect("seed manuscript");
        fs::write(deck_dir.join("slides.md"), "---\nmarp: true\n---\n# Test\n")
            .expect("seed slides");
        fs::write(deck_dir.join("theme.css"), "").expect("seed theme");
    }

    pub fn create_invalid_deck_dir(&self, id: &str) {
        fs::create_dir_all(self.root.join("decks").join(id)).expect("create invalid deck");
    }

    pub fn command(&self) -> Command {
        let mut command = Command::cargo_bin("mlt").expect("binary");
        command.current_dir(&self.root);
        command
    }
}

fn create_workspace(root: &Path) {
    fs::create_dir_all(root.join("decks")).expect("create decks");
    fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = 'fixture'\nversion = '0.0.0'\n",
    )
    .expect("create cargo");
}
