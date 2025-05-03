
= Introduction

== What is libp2p?

libp2p#footnote("https://github.com/libp2p/rust-libp2p") is a modular peer-to-peer networking stack, driven by well-designed specifications with several implementations (Go, JavaScript, *Rust*, C).

Rust implementation of libp2p in particular is being used in notable projects like *IPFS* client, *Lighthouse* Ethereum consensus client, *Filecoin* client, *Substrate* (of Polkadot), and many more.


== Protocol Stack

=== Transport Layer

- TCP, QUIC, WebSocket
- Multiplexing (Yamux, mplex)
- Security (Noise)

=== Peer Discovery

- Kademlia DHT
- mDNS
- Rendezvous

=== Peer Communication

- GossipSub
- Request-Response
- DCutR


