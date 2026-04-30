use std::path::Path;

use crate::error::AppResult;
use crate::workspace::{self, Workspace};

pub fn run(root: &Path, id: &str) -> AppResult<Workspace> {
    workspace::create(root, id)
}
