use clap::{Parser};
use csc::{cli::CliArgs, get_info, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    get_info(&args.name).await?;

    Ok(())
}
