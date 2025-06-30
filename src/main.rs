//! UIP Node - Main Entry Point

mod peer_discovery;
mod content_storage;
mod blockchain_dns;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "uip-node")]
#[command(about = "Ungovernable Internet Protocol Node", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the UIP node
    Start {},
    /// Show node status
    Status {},
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Start {}) => {
            println!("Starting UIP node...");
            // TODO: Initialize peer discovery, storage, DNS, etc.
        }
        Some(Commands::Status {}) => {
            println!("UIP node status: (not implemented)");
        }
        None => {
            println!("No command provided. Use --help for options.");
        }
    }
}
