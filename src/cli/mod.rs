use clap::{Parser, Subcommand};

use crate::app::context::Context;
use crate::app::{create, run as run_deck};
use crate::error::{AppError, AppResult};
use crate::marp::Format;
use std::path::PathBuf;

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
    #[command(about = "Create a new deck scaffold", visible_alias = "cr")]
    Create {
        #[arg(help = "New lower-kebab-case deck id")]
        id: String,
    },
    #[command(about = "Export deck slides into deck artifacts", visible_alias = "r")]
    Run {
        #[arg(help = "Path to deck directory")]
        path: PathBuf,
        #[arg(long, help = "Export PDF")]
        pdf: bool,
        #[arg(long, help = "Export HTML")]
        html: bool,
        #[arg(long, help = "Export PPTX")]
        pptx: bool,
    },
}

pub fn run() {
    let cli = Cli::parse();
    let context = match Context::discover() {
        Ok(context) => context,
        Err(error) => exit_with_error(error),
    };

    let result = execute_command(&context, cli.command);

    if let Err(error) = result {
        exit_with_error(error);
    }
}

fn execute_command(context: &Context, command: Command) -> AppResult<()> {
    match command {
        Command::Create { id } => {
            let workspace = create::run(&context.root, &id)?;
            println!("{}", workspace.deck_dir.display());
            Ok(())
        }
        Command::Run {
            path,
            pdf,
            html,
            pptx,
        } => {
            let selected_formats = select_formats(pdf, html, pptx);
            let exported = run_deck::run(&path, &selected_formats)?;
            for path in exported {
                println!("{}", path.display());
            }
            Ok(())
        }
    }
}

fn exit_with_error(error: AppError) -> ! {
    eprintln!("Error: {error}");
    std::process::exit(1);
}

fn select_formats(pdf: bool, html: bool, pptx: bool) -> Vec<Format> {
    let mut formats = Vec::new();
    if pdf {
        formats.push(Format::Pdf);
    }
    if html {
        formats.push(Format::Html);
    }
    if pptx {
        formats.push(Format::Pptx);
    }
    if formats.is_empty() {
        return Format::ALL.to_vec();
    }
    formats
}

#[cfg(test)]
mod tests {
    use super::select_formats;
    use crate::marp::Format;

    #[test]
    fn select_formats_defaults_to_all() {
        assert_eq!(select_formats(false, false, false), Format::ALL);
    }

    #[test]
    fn select_formats_picks_requested_flags_in_fixed_order() {
        assert_eq!(
            select_formats(true, false, true),
            vec![Format::Pdf, Format::Pptx]
        );
        assert_eq!(
            select_formats(false, true, true),
            vec![Format::Html, Format::Pptx]
        );
    }
}
