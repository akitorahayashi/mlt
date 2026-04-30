use std::path::Path;

use crate::error::AppResult;
use crate::workspace::{self, Entry};

pub fn run(root: &Path) -> AppResult<Vec<Entry>> {
    workspace::list(root)
}
