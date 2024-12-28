use std::{f32::EPSILON, process::exit};

use clap::{Parser, Subcommand}; // 引入 clap 库的必要组件

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// comandslist
#[derive(Subcommand)]
enum Commands {
    /// Get a string value associated with a string key.
    Get { key: String },
    ///Set a string vale associated with a string key.
    Set { key: String, value: String },
    ///Remove a string value associated with a string key.
    Rm { key: String },
}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Get { key: _ } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Set { key: _, value: _ } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Rm { key: _ } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
