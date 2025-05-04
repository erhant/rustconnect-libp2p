= Caveats

== Authentication in GossipSub

GossipSub has several message authentication methods, with these enum variants:

- `Signed(Keypair)`
- `Author(PeerId)`
- `RandomAuthor`
- `Anonymous`

`Signed` is the most secure one, enabling a cryptographic guarantee that the message received originates from that peer. However...

#image("./img/flamegraph.svg")


== `ulimit`

LibP2P will use a lot of file descriptors when the network is scaled up, so you may need to increase the `ulimit` on your system.

```sh
ulimit -n 12345
```

You will likely need to do this on the client code automatically:

#text(size: 0.8em)[
  ```rust
  let (soft, hard) = rlimit::Resource::NOFILE
        .get()
        .unwrap_or((DEFAULT_SOFT_LIMIT, DEFAULT_HARD_LIMIT));
  let target_soft = hard / 10;
  if soft < target_soft {
      if let Err(e) = rlimit::Resource::NOFILE.set(target_soft, hard) {
          // log...
      }
  }
  ```
]

== Memory Backpressure

Suppose you have a single-thread for handling `SwarmEvent`s, and you are doing heavy work on each received `Request` event. Swarm events will keep coming, and the memory will keep growing until it runs out of memory.

To prevent this, you can use a `mpsc` channel to send the events to a worker thread for processing. This way, the main thread can continue to receive events without blocking.

#text(size: 0.8em)[
  ```rust
  pub struct MyClient {
      swarm: Swarm<MyBehaviour>,
      reqres_tx: mpsc::Sender<(PeerId, MyMessage)>,
      cmd_rx: mpsc::Receiver<MyCommand>,
  }
  ```
]

`Swarm` can't be sent between threads! We need to do some inter-thread communication if we ever need to use Swarm.
We give an `mpsc` receiver to the `MyClient` struct, and from a different thread we can
send "commands" to it, paired with a `oneshot` channel to get the result back.

#text(size: 0.85em)[
  ```rust
  let (sender, receiver) = oneshot::channel();
  self.sender
        .send(DriaP2PCommand::IsConnected { peer_id, sender })
        .await?;
  let is_connected = receiver.await?;
  ```
]
