mod cli;
mod core;

use clap::{Args, CommandFactory};
use std::path::PathBuf;

use clap::{command, Command, Parser, Subcommand};
use cli::build::build;

pub static TEMP_DIR: &str = ".tmp";

#[derive(Parser)]
#[command(author,  version, about, long_about = None, arg_required_else_help = true, )]
struct Cli {
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
