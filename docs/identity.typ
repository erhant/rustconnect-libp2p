#import "@preview/touying:0.6.1": pause


== What is a Peer?

A peer is a node in a distributed network that can communicate with other nodes. In `libp2p`, peers are identified by their unique `PeerId`s and `Multiaddr`s.

A peer ID is a cryptographic identifier of a peer in the network, derived from the public key of the peer. LibP2P has two types of keys:

- *Secp256k1*: A widely used curve for ECDSA, often associated with Bitcoin and Ethereum.
- *Ed25519*: A modern curve used for EdDSA, with better speed and security.

Multi-addressing is a flexible addressing scheme used in `libp2p` to represent network addresses. It allows peers to specify multiple protocols and transport layers in a single address format.

- `/ip4/3.7.24.28/tcp/1234`
- `/ip4/6.18.23.23/tcp/4001/p2p/QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N`

