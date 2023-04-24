use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Name of the city. Example: London, Moscow and etc
    #[arg(short, long)]
    pub name: String
}