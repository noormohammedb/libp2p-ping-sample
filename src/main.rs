use futures::StreamExt;
use libp2p::swarm::{keep_alive, NetworkBehaviour, SwarmBuilder, SwarmEvent};
use libp2p::{identity, ping, Multiaddr, PeerId};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
  println!("libp2p ping sample");

  let local_key = identity::Keypair::generate_ed25519();
  let local_peer_id = PeerId::from(local_key.public());
  println!("Local peer id: {local_peer_id:?}");

  let transport = libp2p::development_transport(local_key).await?;

  let behavior = Behavior::default();

  let mut swarm = SwarmBuilder::with_async_std_executor(transport, behavior, local_peer_id).build();

  swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

  if let Some(addr) = std::env::args().nth(1) {
    let remote: Multiaddr = addr.parse()?;
    swarm.dial(remote)?;
    println!("Dialed {addr}");
  }

  loop {
    match swarm.select_next_some().await {
      SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
      SwarmEvent::Behaviour(event) => println!("{event:?}"),
      _ => {}
    }
  }
}

#[derive(NetworkBehaviour, Default)]
struct Behavior {
  keep_alaiv: keep_alive::Behaviour,
  ping: ping::Behaviour,
}
