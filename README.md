# UIP Node

A censorship-resistant, peer-to-peer Rust node for the Ungovernable Internet Protocol (UIP): decentralized content hosting, blockchain DNS, and simulated incentives.

## Features

- Peer discovery (libp2p, mDNS, UDP broadcast)
- Content storage (persistent, SQLite backend)
- Blockchain DNS interaction (simulated)
- Command-line interface (CLI) with multiple commands
- Incentive mechanism (simulated rewards)

## Getting Started

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Clone this repo
3. Build and run:

   ```sh
   cargo run -- start
   ```

## CLI Commands

- `start`    : Start the UIP node (peer discovery, content storage, blockchain DNS, incentives)
- `status`   : Show node status (not yet implemented)
- `discover` : Run peer discovery only
- `storage`  : Store and retrieve content (persistent, SQLite)
- `dns`      : Simulate blockchain DNS
- `storage store <hash> <data>` : Store content with a hash and data string
- `storage get <hash>`          : Retrieve content by hash

Example usage:

```sh
cargo run -- start
cargo run -- storage store myhash "Hello, UIP!"
cargo run -- storage get myhash
```

## Networking

- libp2p-based peer-to-peer networking with mDNS discovery is now integrated.
- On `cargo run -- start`, the node will listen for and discover peers on the local network.

## Roadmap

- [x] Implement peer discovery (basic UDP broadcast, libp2p)
- [x] Add content storage and retrieval (in-memory, persistent/SQLite)
- [x] Integrate blockchain DNS (simulated)
- [x] Add CLI commands for each module (`start`, `status`, `discover`, `storage`, `dns`)
- [x] Use persistent storage (SQLite database)
- [x] Implement networking (libp2p TCP, mDNS)
- [x] Simulated incentive mechanism (token rewards)
- [ ] Integrate with real smart contract (see Solidity code)
- [ ] Full integration with other UIP components
- [ ] Expand CLI, tests, and documentation
