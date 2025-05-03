= Usage

== Behaviour

All modules we have talked about so far belong to a "behaviour" struct.

```rust
use libp2p::{gossipsub, identify, mdns};
use libp2p::swarm::NetworkBehaviour;

#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub identify: identify::Behaviour,
    // ...
}
```

This `derive` macro automatically implements the `NetworkBehaviour` trait for our struct, allowing us to use it as a behaviour in our libp2p application.

The `NetworkBehaviour` trait also provides a way to handle events that occur in the network. For example, when a new peer connects or disconnects, we can handle these events in our behaviour struct.

== Swarm

We then provide the behaviours to a `Swarm` struct, which is responsible for managing the network connections and communication between peers.

```rust
use libp2p::swarm::Swarm;

pub struct MyClient {
    swarm: Swarm<MyBehaviour>,
    // ...
}
```

Swarm then provides us `behaviour()` and `behaviour_mut()` methods to access the behaviours, e.g. `swarm.behaviour_mut().gossipsub`.

== Events

LibP2P handles everything behind the scenes, and all providers communicate with each other through `SwarmEvent`s:

#text(size: 0.9em)[
  ```rust
    match self.swarm.select_next_some().await {
      SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
        println!("Disconnected from peer: {:?}, cause: {:?}", peer_id, cause);
      }
      SwarmEvent::Behaviour(MyBehaviourEvent::GossipSub(gossip_event)) => {}
      SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns_event)) => {}
      // and so on...
    }
  ```
]


// talk about gossipsub message auth and flamegraph
// talk about multi-threading
// talk about command with mpsc and oneshot
