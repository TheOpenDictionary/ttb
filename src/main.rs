mod cli;
mod core;

use std::path::PathBuf;

use clap::{command, Parser, Subcommand};
use cli::build::build;

pub static TEMP_DIR: &str = ".tmp";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Downloads the latest Tatoeba data and builds a local, compressed index
    Build,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build) => {
            build().await.unwrap();
        }
        None => {}
    }
}
