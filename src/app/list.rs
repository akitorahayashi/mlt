use std::path::Path;

use crate::error::AppResult;
use crate::workspace;

pub fn run(root: &Path) -> AppResult<Vec<String>> {
    workspace::list(root)
}
