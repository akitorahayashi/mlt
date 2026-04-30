pub mod app;
pub mod cli;
pub mod error;
pub mod marp;
pub mod workspace;

pub fn run() {
    cli::run();
}
