use libp2p::swarm::{keep_alive, NetworkBehaviour, SwarmBuilder};
use libp2p::{identity, ping, PeerId};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
  println!("Hello, world!");

  let local_key = identity::Keypair::generate_ed25519();
  let local_peer_id = PeerId::from(local_key.public());
  println!("Local peer id: {local_peer_id:?}");

  let transport = libp2p::development_transport(local_key).await?;

  let behavior = Behavior::default();

  let mut swarm = SwarmBuilder::with_async_std_executor(transport, behavior, local_peer_id).build();

  Ok(())
}

#[derive(NetworkBehaviour, Default)]
struct Behavior {
  keep_alaiv: keep_alive::Behaviour,
  ping: ping::Behaviour,
}
