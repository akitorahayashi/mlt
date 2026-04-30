use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use tempfile::TempDir;

use marp_pj::workspace;

pub struct TestContext {
    _temp_dir: TempDir,
    root: PathBuf,
}

#[allow(dead_code)]
impl TestContext {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("temp dir");
        let root = temp_dir.path().join("workspace");
        fs::create_dir_all(root.join("decks")).expect("create decks");
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = 'fixture'\nversion = '0.0.0'\n",
        )
        .expect("create cargo");
        Self {
            _temp_dir: temp_dir,
            root,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn create_deck(&self, id: &str) {
        workspace::create(&self.root, id).expect("create deck");
    }

    pub fn create_invalid_deck_dir(&self, id: &str) {
        fs::create_dir_all(self.root.join("decks").join(id)).expect("create invalid deck");
    }

    pub fn command(&self) -> Command {
        let mut command = Command::cargo_bin("marp-pj").expect("binary");
        command.current_dir(&self.root);
        command
    }
}
