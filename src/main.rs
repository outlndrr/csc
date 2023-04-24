use clap::{Parser};
use csc::{cli::CliArgs, get_summary, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    get_summary(&args.name).await?;

    Ok(())
}
