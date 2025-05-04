= Usage

== Behaviour

All modules we have talked about so far belong to a "behaviour" struct.

#columns(2)[
  #text(size: 0.75em)[

    ```rust
    use libp2p::{gossipsub, identify, mdns};
    use libp2p::swarm::NetworkBehaviour;

    #[derive(NetworkBehaviour)]
    struct MyBehaviour {
        pub gossipsub: gossipsub::Behaviour,
        pub mdns: mdns::tokio::Behaviour,
        pub identify: identify::Behaviour,
        // ...
    }
    ```
  ]

  #colbreak()
  This `derive` macro automatically implements the `NetworkBehaviour` trait for our struct, allowing us to use it as a behaviour in our libp2p application.

]

== Swarm

We then provide the behaviours to a `Swarm` struct, which is responsible for managing the network connections and communication between peers.

#text(size: 0.9em)[
  ```rust
  use libp2p::swarm::Swarm;

  pub struct MyClient {
      swarm: Swarm<MyBehaviour>,
      // ...
  }
  ```
]

Swarm then provides us `behaviour()` and `behaviour_mut()` methods to access the behaviours, e.g. `swarm.behaviour_mut().gossipsub`.

#import "@preview/touying:0.6.1": pause

== Connection Configuration

Connection configuration is done through the `SwarmBuilder` struct. You can configure the transport, identity, and other parameters of the connection.

#text(size: 0.9em)[
  ```rust
  let keypair = libp2p::identity::Keypair::generate_secp256k1();
  let swarm = libp2p::SwarmBuilder::with_existing_identity(keypair)
      .with_tokio()
      .with_tcp(
          tcp::Config::default(),
          noise::Config::new,
          yamux::Config::default,
      )?
      .with_behaviour(|key| Ok(ChatBehaviour::new(key).unwrap()))
      .unwrap().build();
  ```
]

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

== Metrics

Suppose you want to launch an HTTP server on the side to expose metrics about your P2P client. You can use an `Arc<RwLock<MyMetrics>>` and share it with the HTTP server thread and the P2P thread.

#text(size: 0.85em)[
  ```rust
  tokio::select! {
      event = self.swarm.select_next_some() => {/*...*/}
      _ = metrics_interval.tick() => {
         let mut metrics = self.metrics.write().await;
         // ...
      }
  }
  ```
]

Then within your HTTP server thread, you can read the metrics:

#text(size: 0.85em)[
  ```rust
  let metrics = metrics.read().await;
  // ...
  ```
]
