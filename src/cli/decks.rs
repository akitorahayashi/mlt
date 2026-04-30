use clap::Subcommand;

use crate::app::context::Context;
use crate::app::decks::{create, list, show};
use crate::error::AppResult;

#[derive(Subcommand)]
pub enum DecksCommand {
    #[command(about = "List valid decks")]
    List,
    #[command(about = "Show resolved paths for a deck")]
    Show {
        #[arg(help = "Deck id or deck directory path")]
        reference: String,
    },
    #[command(about = "Create a new deck scaffold")]
    Create {
        #[arg(help = "New lower-kebab-case deck id")]
        id: String,
    },
}

pub fn run(context: &Context, command: DecksCommand) -> AppResult<()> {
    match command {
        DecksCommand::List => {
            let entries = list::run(&context.root)?;
            if entries.is_empty() {
                println!("No valid decks found");
                return Ok(());
            }

            for entry in entries {
                println!("{}\t{}", entry.id, entry.title);
            }
            Ok(())
        }
        DecksCommand::Show { reference } => {
            let workspace = show::run(&context.root, &reference)?;
            println!("deck-id: {}", workspace.manifest.deck_id);
            println!("title: {}", workspace.manifest.title);
            println!("deck-dir: {}", workspace.deck_dir.display());
            println!("manifest: {}", workspace.manifest_path.display());
            println!("manuscript: {}", workspace.manuscript_path.display());
            println!("slides: {}", workspace.slides_path.display());
            println!("theme: {}", workspace.theme_path.display());
            println!("output-dir: {}", workspace.output_dir.display());
            Ok(())
        }
        DecksCommand::Create { id } => {
            let workspace = create::run(&context.root, &id)?;
            println!("{}", workspace.deck_dir.display());
            Ok(())
        }
    }
}
