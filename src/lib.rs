pub mod app;
pub mod cli;
pub mod deck_layout;
pub mod error;
pub mod marp;

pub fn run() {
    cli::run();
}
