use std::path::Path;

use crate::error::AppResult;
use crate::workspace::{self, Workspace};

pub fn run(root: &Path, reference: &str) -> AppResult<Workspace> {
    workspace::resolve(root, reference)
}
