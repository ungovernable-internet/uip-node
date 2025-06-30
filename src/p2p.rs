//! libp2p-based peer-to-peer networking for UIP Node

use futures_util::StreamExt;
use libp2p::{
    core::upgrade,
    identity, mdns, noise, PeerId, Swarm, Multiaddr,
    swarm::SwarmEvent,
    tcp, yamux,
    Transport,
};
use std::error::Error;

pub async fn start_libp2p_node() -> Result<(), Box<dyn Error>> {
    // Generate a random keypair for this node
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {}", peer_id);

    // Set up transport (TCP + Noise + Yamux)
    let transport = tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::Config::new(&id_keys)?)
        .multiplex(yamux::Config::default())
        .boxed();

    // mDNS for local peer discovery
    let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id)?;

    // Build the Swarm
    let mut swarm = Swarm::new(transport, mdns, peer_id, libp2p::swarm::Config::with_tokio_executor());

    // Listen on all interfaces and a random OS-assigned port
    let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse()?;
    swarm.listen_on(listen_addr)?;

    // Main event loop
    println!("libp2p node running. Waiting for peers...");
    loop {
        match StreamExt::next(&mut swarm).await {
            Some(SwarmEvent::Behaviour(libp2p::mdns::Event::Discovered(peers))) => {
                for (peer, _addr) in peers {
                    println!("Discovered peer: {}", peer);
                }
            }
            Some(SwarmEvent::Behaviour(libp2p::mdns::Event::Expired(expired))) => {
                for (peer, _addr) in expired {
                    println!("Expired peer: {}", peer);
                }
            }
            Some(SwarmEvent::NewListenAddr { address, .. }) => {
                println!("Listening on {:?}", address);
            }
            _ => {}
        }
    }
}
