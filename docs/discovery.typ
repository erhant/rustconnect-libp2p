#import "@preview/touying:0.6.1": pause

== Peer Discovery

#columns(2)[
  === Kademlia DHT

  Kademlia is a Distributed Hash Table (DHT) protocol used for peer discovery and data storage in decentralized networks. It allows peers to efficiently locate other peers and resources in the network.

  #colbreak()
  === Multicast DNS (mDNS)

  mDNS is a zero-configuration protocol that allows devices on a local network to discover each other, just via the router.

  === Rendezvous
  A server simply collects peer information and newly-joined peers get all that information from this server.

]


// - *Peer Discovery*: Peers can find each other using the DHT, even if they are behind NATs or firewalls.
// - *Data Storage*: Kademlia allows peers to store and retrieve data in a distributed manner, ensuring that data is available even if some peers go offline.
// - *Routing*: Kademlia uses a XOR-based distance metric to efficiently route messages between peers, ensuring low-latency communication.
// - *Replication*: Kademlia supports data replication, ensuring that data is available even if some peers go offline.
// - *Fault Tolerance*: Kademlia is designed to be resilient to node failures, ensuring that the network remains operational even in the presence of failures.
// - *Scalability*: Kademlia can scale to large networks, making it suitable for applications with many peers.
// - *Security*: Kademlia includes mechanisms for securing communication between peers, ensuring that data is not tampered with or intercepted during transmission.




// - *Local Network Discovery*: mDNS is primarily used for local network discovery, allowing devices to find each other without the need for a central server.
// - *Zero Configuration*: mDNS is designed to work without any configuration, making it easy to set up and use.
// - *Human-Readable Names*: mDNS allows devices to be identified using human-readable names, making it easier for users to find and connect to devices.
// - *Service Discovery*: mDNS can be used to discover services offered by devices on the local network, such as printers, file shares, and media servers.
