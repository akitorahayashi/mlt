use clap::{Parser, Subcommand, ValueEnum};

use crate::app::context::Context;
use crate::app::{create, list, run as run_deck};
use crate::error::{AppError, AppResult};
use crate::marp::Format;

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
    #[command(about = "List valid deck ids")]
    List,
    #[command(about = "Create a new deck scaffold")]
    Create {
        #[arg(help = "New lower-kebab-case deck id")]
        id: String,
    },
    #[command(about = "Export deck slides into deck artifacts")]
    Run {
        #[arg(help = "Deck id")]
        id: String,
        #[arg(long, value_enum, help = "Optional output format")]
        format: Option<RunFormat>,
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
        Command::List => {
            let deck_ids = list::run(&context.root)?;
            if deck_ids.is_empty() {
                println!("No valid decks found");
                return Ok(());
            }
            for deck_id in deck_ids {
                println!("{deck_id}");
            }
            Ok(())
        }
        Command::Create { id } => {
            let workspace = create::run(&context.root, &id)?;
            println!("{}", workspace.deck_dir.display());
            Ok(())
        }
        Command::Run { id, format } => {
            let exported = run_deck::run(&context.root, &id, format.map(RunFormat::into_format))?;
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

#[derive(Debug, Clone, Copy, ValueEnum)]
enum RunFormat {
    Pdf,
    Html,
    Png,
    Pptx,
}

impl RunFormat {
    fn into_format(self) -> Format {
        match self {
            Self::Pdf => Format::Pdf,
            Self::Html => Format::Html,
            Self::Png => Format::Png,
            Self::Pptx => Format::Pptx,
        }
    }
}
