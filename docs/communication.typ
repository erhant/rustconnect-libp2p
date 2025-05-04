#import "@preview/touying:0.6.1": pause, slide

== Communication

LibP2P has two notable communication methods:

- *GossipSub*: Nodes use "gossip protocols" to distribute messages across the network efficiently; useful for broadcasting messages to multiple peers.
- *Request-Response*: Nodes can send requests to each other and receive responses, allowing for more structured communication patterns.

It also provides a method to bypass *NAT* problems between peers.

==
#columns(2)[
  === GossipSub

  GossipSub is a publish-subscribe protocol that allows nodes to efficiently disseminate messages across the network.

  Uses a combination of gossiping and topic-based subscriptions to ensure that messages reach interested peers while minimizing network overhead, while keeping track of peer scores to prioritize messages from trusted peers.

  #colbreak()
  === Request/Response Protocol

  Allow nodes to send requests to each other and receive responses (one-to-one). Useful for scenarios where a node needs to query another node for specific information or perform a remote procedure call (RPC).

]

=== NAT & DCutR

LibP2P provides several mechanisms for NAT traversal, allowing peers behind NATs to communicate with each other.

DCutR (Direct Connection Upgrade through Relay) protocol is designed to facilitate direct connections between peers behind NATs.
- *Hole punching*: DCutR uses hole punching techniques to establish direct connections between peers, even if they are behind NATs.
- *Relay servers*: In cases where direct connections cannot be established, DCutR can use relay servers to facilitate communication between peers.

// == NAT Problem

// A classic problem of peer-to-peer networks is the *NAT traversal* problem. NAT (Network Address Translation) allows multiple devices on a local network to share a single public IP address.

// === Public - Private

// For example, consider a device behind a NAT at local IP `192.168.1.5:3000` trying to communicate with a public device at `203.0.113.4:8080`. When the local device sends a packet:

// 1. The packet first goes to the NAT router (`192.168.1.1`)
// 2. NAT router changes source address from `192.168.1.5:3000` to `71.198.63.2:5678` (public IP)
// 3. Packet reaches destination `203.0.113.4:8080`
// 4. Return packets must go through the same NAT mapping to reach local device

// === Private - Private

// Now consider two devices behind NATs trying to communicate:

// 1. Device A (`192.168.1.5:3000`) behind NAT A (`71.198.63.2`) tries to reach Device B (`192.168.2.7:4000`) behind NAT B (`81.201.44.3`)
// 2. Neither NAT knows how to forward incoming connections without *prior outbound traffic*
// 3. Both NATs block incoming connections by default
// 4. This creates a "symmetric NAT" situation where direct communication is not possible without:
//   - Port-forwarding // familiar from gaming
//   - Relay servers
//   - Hole punching
