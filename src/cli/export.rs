use clap::Args;

use crate::app::context::Context;
use crate::app::export::managed;
use crate::error::AppResult;
use crate::marp::Target;

#[derive(Args)]
pub struct ExportArgs {
    #[arg(value_enum, help = "Export format")]
    format: Target,
    #[arg(help = "Deck id or deck directory path")]
    reference: String,
}

pub fn run(context: &Context, args: ExportArgs) -> AppResult<()> {
    let exported = managed::run(&context.root, &args.reference, args.format)?;
    for path in exported {
        println!("{}", path.display());
    }
    Ok(())
}
