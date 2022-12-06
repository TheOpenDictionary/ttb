mod cli;
mod core;

use clap::{command, Parser, Subcommand};
use cli::{build::build, search::search};

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
    /// Searches the local Tatoeba index if one exists
    Search {
        /// Limit to the top k results
        #[arg(long, short, default_value_t = 10)]
        limit: usize,
        /// Three-letter language code to search by (eng, cmn, etc.)
        #[arg(short = 'g', long)]
        lang: Option<String>,
        /// Three-letter language code of the language to translate the strings into. Only translatable sentences will be shown.
        #[arg(short, long)]
        trans: Option<String>,
        /// The word or term you're looking for
        #[arg()]
        query: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build) => {
            build().await.unwrap();
        }
        Some(Commands::Search {
            lang,
            trans,
            query,
            limit,
        }) => {
            if let Err(err) = search(query, lang, trans, limit).await {
                println!("{}", err.as_ref());
            }
        }
        None => {}
    }
}
