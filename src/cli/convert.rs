use std::path::PathBuf;

use clap::Args;

use crate::app::context::Context;
use crate::app::export::direct;
use crate::error::AppResult;
use crate::marp::Target;

#[derive(Args)]
pub struct ConvertArgs {
    #[arg(value_enum, help = "Export format")]
    format: Target,
    #[arg(help = "Path to a completed slides.md file")]
    slides_path: PathBuf,
    #[arg(long, help = "Directory for generated files")]
    output_dir: PathBuf,
    #[arg(long, help = "Optional CSS file passed to Marp as the export theme")]
    theme: Option<PathBuf>,
    #[arg(
        long,
        help = "Optional output basename. Defaults to the slides filename stem"
    )]
    basename: Option<String>,
}

pub fn run(context: &Context, args: ConvertArgs) -> AppResult<()> {
    let exported = direct::run(
        &context.root,
        &args.slides_path,
        &args.output_dir,
        args.theme.as_deref(),
        args.basename.as_deref(),
        args.format,
    )?;
    for path in exported {
        println!("{}", path.display());
    }
    Ok(())
}
