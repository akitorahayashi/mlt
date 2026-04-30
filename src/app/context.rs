use std::env;
use std::path::PathBuf;

use crate::error::AppResult;
use crate::workspace;

pub struct Context {
    pub root: PathBuf,
}

impl Context {
    pub fn discover() -> AppResult<Self> {
        let current_dir = env::current_dir()?;
        let root = workspace::locate_root(&current_dir)?;
        Ok(Self { root })
    }
}
