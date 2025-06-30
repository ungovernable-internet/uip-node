//! UIP Node - Main Entry Point

mod peer_discovery;
mod content_storage;
mod blockchain_dns;
mod p2p;
mod incentives;

use clap::{Parser, Subcommand, Args};

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
    /// Run peer discovery
    Discover {},
    /// Store content with a hash and data, or retrieve by hash
    Storage(Storage),
    /// Simulate blockchain DNS
    Dns {},
}

#[derive(Parser)]
struct Storage {
    #[command(subcommand)]
    subcommand: StorageSubcommand,
}

#[derive(Subcommand)]
enum StorageSubcommand {
    /// Store content: provide a hash and data string
    Store(StoreArgs),
    /// Retrieve content by hash
    Get(GetArgs),
}

#[derive(Args)]
struct StoreArgs {
    hash: String,
    data: String,
}

#[derive(Args)]
struct GetArgs {
    hash: String,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Start {}) => {
            println!("Starting UIP node...");
            peer_discovery::start_peer_discovery();
            blockchain_dns::start_blockchain_dns();
            incentives::start_incentives();
            // Start libp2p networking (async)
            println!("Starting libp2p networking...");
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime")
                .block_on(async {
                    if let Err(e) = p2p::start_libp2p_node().await {
                        eprintln!("libp2p error: {}", e);
                    }
                });
        }
        Some(Commands::Status {}) => {
            println!("UIP node status: (not implemented)");
        }
        Some(Commands::Discover {}) => {
            peer_discovery::start_peer_discovery();
        }
        Some(Commands::Storage(storage)) => {
            use content_storage::ContentStore;
            let store = ContentStore::new("uip_content.db").expect("Failed to open DB");
            match &storage.subcommand {
                StorageSubcommand::Store(args) => {
                    store.store(&args.hash, args.data.as_bytes()).expect("Store failed");
                    println!("Stored content for hash: {}", args.hash);
                }
                StorageSubcommand::Get(args) => {
                    match store.retrieve(&args.hash) {
                        Ok(Some(data)) => println!("Retrieved content: {:?}", String::from_utf8_lossy(&data)),
                        Ok(None) => println!("No content found for hash: {}", args.hash),
                        Err(e) => println!("Error retrieving content: {}", e),
                    }
                }
            }
        }
        Some(Commands::Dns {}) => {
            blockchain_dns::start_blockchain_dns();
        }
        None => {
            println!("No command provided. Use --help for options.");
        }
    }
}
