mod commands;
mod library;

pub const LIBRARY_PATH: &str = "library.bib";

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A command line research paper manager
#[derive(Parser)]
#[command(name = "papr", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a citation and pdf by the paper's DOI
    Add {
        /// The DOI of the paper
        doi: String,
    },

    /// Add a citation from a BibTeX entry
    Import {
        /// The file containing the BibTeX entry
        entry: PathBuf,
    },

    /// Show all papers in the library
    List,

    /// Find a paper using identifiers
    Search {
        /// Keywords used for searching
        query: Vec<String>,
    },

    /// Delete a paper entry from the library
    Remove {
        /// The paper's generated key
        key: String,
    },

    /// Open a pdf version of a paper
    Open {
        /// The paper's generated key
        key: String,
    },

    /// Open the markdown note file for a paper
    Note {
        /// The paper's generated key
        key: String,
    },

    /// Generate a BibTeX citation for a paper
    Export {
        /// The paper's generated key
        key: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { doi } => commands::add(&doi),
        Commands::Import { entry } => commands::import(&entry),
        Commands::List => commands::list(),
        Commands::Search { query } => commands::search(&query),
        Commands::Remove { key } => commands::remove(&key),
        Commands::Open { key } => commands::open(&key),
        Commands::Note { key } => commands::note(&key),
        Commands::Export { key } => commands::export(&key),
    }
}
