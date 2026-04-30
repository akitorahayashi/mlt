mod convert;
mod decks;
mod export;

use clap::{Parser, Subcommand};

use crate::app::context::Context;
use crate::error::AppError;

#[derive(Parser)]
#[command(name = "marp-pj")]
#[command(version)]
#[command(about = "Manage local Marp decks and exports", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Manage deck directories", visible_alias = "d")]
    Decks {
        #[command(subcommand)]
        command: decks::DecksCommand,
    },
    #[command(about = "Export a managed deck")]
    Export(export::ExportArgs),
    #[command(about = "Convert a completed slides.md directly")]
    Convert(convert::ConvertArgs),
}

pub fn run() {
    let cli = Cli::parse();
    let context = match Context::discover() {
        Ok(context) => context,
        Err(error) => exit_with_error(error),
    };

    let result = match cli.command {
        Command::Decks { command } => decks::run(&context, command),
        Command::Export(args) => export::run(&context, args),
        Command::Convert(args) => convert::run(&context, args),
    };

    if let Err(error) = result {
        exit_with_error(error);
    }
}

fn exit_with_error(error: AppError) -> ! {
    eprintln!("Error: {error}");
    std::process::exit(1);
}
